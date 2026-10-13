use std::sync::Arc;

use anyhow::Result;
use thiserror::Error;
use web_sys::{
    js_sys::{Reflect, Uint8Array},
    wasm_bindgen::{JsCast, JsValue},
    window,
    File,
    FileSystemDirectoryHandle,
    FileSystemFileHandle,
    FileSystemGetFileOptions,
    FileSystemWritableFileStream,
    Request,
    RequestInit,
    RequestMode,
    Response
};
use dapa_common::crypto::ecdlp::{self, ECDLPTables};
use wasm_bindgen_futures::JsFuture;
use log::{info, warn};

use super::*;

#[derive(Debug, Error)]
pub enum PrecomputedTablesError {
    #[error("window error: {0}")]
    Window(String),
    #[error("file system error: {0}")]
    FileSystem(String),
    #[error("directory error: {0}")]
    Directory(String),
    #[error("file error: {0}")]
    File(String),
    #[error("writable file error: {0}")]
    WritableFile(String),
    #[error("write error: {0}")]
    Write(String),
    #[error("write result error: {0}")]
    WriteResult(String),
    #[error("write close error: {0}")]
    WriteClose(String),
    #[error("into file error: {0}")]
    IntoFile(String),
    #[error("array buffer error: {0}")]
    ArrayBuffer(String),
    #[error("reflect error: {0}")]
    Reflect(String),
}

macro_rules! js_future {
    ($e:expr) => {
        JsFuture::from($e).await
    };
}

macro_rules! execute {
    ($e:expr, $err:ident) => {
        js_future!($e).map(|v| v.unchecked_into()).map_err(|e| PrecomputedTablesError::$err(format!("{:?}", e)))
    };
}

// Check if the precomputed tables exists
pub async fn has_precomputed_tables(_: Option<&str>, l1: usize) -> Result<bool> {
    let path = format!("precomputed_tables_{l1}.bin");

    let window = window().ok_or(PrecomputedTablesError::Window("window not found in context".to_owned()))?;
    let navigator = window.navigator();
    let storage = navigator.storage();
    // On Safari, if directory is not available, it means he is in an ephemeral context (private tab), which is not supported by WebKit
    let directory: FileSystemDirectoryHandle = match execute!(storage.get_directory(), Directory) {
        Ok(directory) => directory,
        Err(e) => {
            warn!("Directory not available, precomputed tables cannot be present: {}", e);
            return Ok(false)
        }
    };

    // By default, it will not create a new file false
    // we check if the file exists
    // and expect the file to have the same size as the precomputed tables
    let file_handle: Option<FileSystemFileHandle> = js_future!(directory.get_file_handle(path.as_str()))
        .ok()
        .map(|v| v.unchecked_into());

    if let Some(file_handle) = file_handle {
        // Verify the size of the file
        let file: File = execute!(file_handle.get_file(), IntoFile)?;
        Ok(file.size() as usize == ECDLPTables::get_required_sizes(l1).0)
    } else {
        Ok(false)
    }
}

// Precomputed tables is too heavy to be stored in local Storage, and generating it on the fly would be too slow
// So we will generate it on the server and store it in a file, and then we will read it from the file
pub async fn read_or_generate_precomputed_tables<P: ecdlp::ProgressTableGenerationReportFunction>(_: Option<&str>, l1: usize, progress_report: P, store_on_disk: bool) -> Result<PrecomputedTablesShared> {
    let path = format!("precomputed_tables_{l1}.bin");

    let window = window().ok_or(PrecomputedTablesError::Window("window not found in context".to_owned()))?;
    let navigator = window.navigator();
    let storage = navigator.storage();
    let directory: Result<FileSystemDirectoryHandle, PrecomputedTablesError> = execute!(storage.get_directory(), Directory);

    // By default, it will not create a new file false
    let (file_handle, directory): (Option<FileSystemFileHandle>, Option<FileSystemDirectoryHandle>) = match directory {
        Ok(directory) => (
            js_future!(directory.get_file_handle(path.as_str()))
            .ok()
            .map(|v| v.try_into().ok())
            .flatten(),
            Some(directory)
        ),
        Err(e) => {
            warn!("Directory not available: {}", e);
            (None, None)
        }
    };

    let required_size = ECDLPTables::get_required_sizes(l1).0;

    let tables = match file_handle {
        Some(file_handle) => {
            info!("Loading precomputed tables from {}", path);

            // Read the tables. The bytes stay in a JS ArrayBuffer and are copied
            // ONCE straight into the aligned table memory (no intermediate Vec).
            let file: File = execute!(file_handle.get_file(), IntoFile)?;
            info!("File size: {}", file.size());

            let value: JsValue = execute!(file.array_buffer(), ArrayBuffer)?;
            let stored_len = js_len(&value);
            if stored_len != required_size {
                info!("File stored has an invalid size ({} bytes), trying server download before regenerating...", stored_len);
                match fetch_tables_from_server(l1).await {
                    Ok(downloaded) if js_len(&downloaded) == required_size => {
                        info!("Downloaded {} bytes from server", required_size);
                        let tables = tables_from_js_buffer(l1, &downloaded)?;
                        if store_on_disk {
                            cache_to_file(&file_handle, &downloaded).await;
                        }
                        tables
                    },
                    Ok(downloaded) => {
                        warn!("Downloaded tables file has wrong size ({} bytes), falling back to local generation", js_len(&downloaded));
                        generate_tables(path.as_str(), l1, Some(file_handle), progress_report, store_on_disk).await?
                    },
                    Err(e) => {
                        warn!("Failed to download precomputed tables from server: {}, falling back to local generation", e);
                        generate_tables(path.as_str(), l1, Some(file_handle), progress_report, store_on_disk).await?
                    }
                }
            } else {
                info!("Loading {} bytes", stored_len);
                tables_from_js_buffer(l1, &value)?
            }
        },
        None => {
            // Try downloading a pre-built tables file from our own server first.
            // If the download fails for any reason we fall back to local generation.
            info!("Attempting to download precomputed tables from server");
            match fetch_tables_from_server(l1).await {
                Ok(downloaded) if js_len(&downloaded) == required_size => {
                    info!("Downloaded {} bytes from server", required_size);
                    let tables = tables_from_js_buffer(l1, &downloaded)?;

                    // Best-effort: cache the downloaded bytes so future visits load from disk.
                    if store_on_disk {
                        if let Some(directory) = &directory {
                            let opts = FileSystemGetFileOptions::new();
                            opts.set_create(true);
                            if let Ok(file_handle) = execute!(directory.get_file_handle_with_options(path.as_str(), &opts), File) {
                                let file_handle: FileSystemFileHandle = file_handle;
                                cache_to_file(&file_handle, &downloaded).await;
                            }
                        }
                    }

                    tables
                },
                Ok(downloaded) => {
                    warn!("Downloaded tables file has wrong size ({} bytes), falling back to local generation", js_len(&downloaded));
                    generate_tables_fallback(path.as_str(), l1, directory, progress_report, store_on_disk).await?
                },
                Err(e) => {
                    warn!("Failed to download precomputed tables from server: {}, falling back to local generation", e);
                    generate_tables_fallback(path.as_str(), l1, directory, progress_report, store_on_disk).await?
                }
            }
        }
    };

    Ok(Arc::new(RwLock::new(tables)))
}

// Generate the tables and store them in a file if API is available
async fn generate_tables<P: ecdlp::ProgressTableGenerationReportFunction>(path: &str, l1: usize, file_handle: Option<FileSystemFileHandle>, progress_report: P, store_on_disk: bool) -> Result<ECDLPTables> {
    let tables = ecdlp::ECDLPTables::generate_with_progress_report(l1, progress_report)?;

    let slice = tables.as_slice();
    info!("Precomputed tables generated");

    let res: Option<FileSystemWritableFileStream> = match file_handle {
        Some(file_handle) => if Reflect::has(&file_handle, &JsValue::from_str("createWritable")).map_err(|e| PrecomputedTablesError::Reflect(format!("{:?}", e)))? {
            Some(execute!(file_handle.create_writable(), WritableFile)?)
        } else {
            None
        },
        None => None
    };

    if let Some(writable) = res.filter(|_| store_on_disk) {
        info!("Writing precomputed tables to {} with {} bytes", path, slice.len());
        // We are forced to copy the slice to a buffer
        // which means we are using twice the memory
        let buffer = Uint8Array::new_with_length(slice.len() as u32);
        buffer.copy_from(slice);

        let promise = writable.write_with_buffer_source(&buffer).map_err(|e| PrecomputedTablesError::Write(format!("{:?}", e)))?;
        let _: JsValue = execute!(promise, WriteResult)?;
        let _: JsValue = execute!(writable.close(), WriteClose)?;
    } else {
        warn!("Failed to create writable file stream, precomputed tables will not be stored");
    }

    Ok(tables)
}

// Fetch a pre-built tables file from our own server (same-origin, so no CORS
// setup is needed as long as it's served from this site's own domain).
async fn fetch_tables_from_server(l1: usize) -> Result<JsValue, PrecomputedTablesError> {
    let url = format!("/tables/precomputed_tables_{l1}.bin");

    let window = window().ok_or(PrecomputedTablesError::Window("window not found in context".to_owned()))?;

    let opts = RequestInit::new();
    opts.set_method("GET");
    opts.set_mode(RequestMode::SameOrigin);

    let request = Request::new_with_str_and_init(&url, &opts)
        .map_err(|e| PrecomputedTablesError::FileSystem(format!("failed to build request: {:?}", e)))?;

    let resp_value: JsValue = execute!(window.fetch_with_request(&request), FileSystem)?;
    let resp: Response = resp_value.dyn_into()
        .map_err(|e| PrecomputedTablesError::FileSystem(format!("bad response object: {:?}", e)))?;

    if !resp.ok() {
        return Err(PrecomputedTablesError::FileSystem(format!("server returned status {}", resp.status())));
    }

    let array_buffer_promise = resp.array_buffer()
        .map_err(|e| PrecomputedTablesError::ArrayBuffer(format!("{:?}", e)))?;
    let array_buffer: JsValue = execute!(array_buffer_promise, ArrayBuffer)?;

    Ok(array_buffer)
}

// Same local-generation logic as before, just pulled out into its own
// function so the download path above can fall back to it cleanly.
async fn generate_tables_fallback<P: ecdlp::ProgressTableGenerationReportFunction>(path: &str, l1: usize, directory: Option<FileSystemDirectoryHandle>, progress_report: P, store_on_disk: bool) -> Result<ECDLPTables> {
    info!("Generating precomputed tables locally");
    let opts = FileSystemGetFileOptions::new();
    opts.set_create(true);

    let file_handle: Option<FileSystemFileHandle> = match directory {
        Some(directory) => Some(execute!(directory.get_file_handle_with_options(path, &opts), File)?),
        None => None
    };
    generate_tables(path, l1, file_handle, progress_report, store_on_disk).await
}

// Length in bytes of a JS ArrayBuffer (no copy).
fn js_len(buffer: &JsValue) -> usize {
    Uint8Array::new(buffer).length() as usize
}

// Build the tables from a JS ArrayBuffer with a single copy, directly into the
// 32-byte aligned table memory. The old path did `to_vec()` and then
// `ECDLPTables::from_bytes`, which held two full copies (~700MB for L1=26) in
// WASM memory at once and aborted with "unreachable" on allocation failure.
fn tables_from_js_buffer(l1: usize, buffer: &JsValue) -> Result<ECDLPTables, PrecomputedTablesError> {
    let src = Uint8Array::new(buffer);
    let required = ECDLPTables::get_required_sizes(l1).0;
    if src.length() as usize != required {
        return Err(PrecomputedTablesError::ArrayBuffer(format!("tables size mismatch: got {} expected {}", src.length(), required)));
    }
    let mut tables = ECDLPTables::empty(l1);
    let dest = tables.as_mut_slice();
    if dest.len() != required {
        return Err(PrecomputedTablesError::ArrayBuffer(format!("table buffer size mismatch: {} vs {}", dest.len(), required)));
    }
    src.copy_to(dest);
    Ok(tables)
}

// Best-effort write of the downloaded bytes to OPFS. Writes the JS buffer
// directly (no extra copy). Failures are logged, never fatal.
async fn cache_to_file(file_handle: &FileSystemFileHandle, buffer: &JsValue) {
    if !Reflect::has(file_handle, &JsValue::from_str("createWritable")).unwrap_or(false) {
        warn!("createWritable not available, tables will not be cached");
        return;
    }
    let writable: FileSystemWritableFileStream = match execute!(file_handle.create_writable(), WritableFile) {
        Ok(w) => w,
        Err(e) => { warn!("Could not open tables cache for writing: {}", e); return; }
    };
    let view = Uint8Array::new(buffer);
    match writable.write_with_buffer_source(&view) {
        Ok(promise) => {
            let _: Result<JsValue, PrecomputedTablesError> = execute!(promise, WriteResult);
            let _: Result<JsValue, PrecomputedTablesError> = execute!(writable.close(), WriteClose);
            info!("Cached downloaded tables to local storage for next visit");
        }
        Err(e) => warn!("Failed to write tables cache: {:?}", e),
    }
}
