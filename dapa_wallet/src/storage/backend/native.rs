pub use sled::{Db, Tree};

use std::{
    fs,
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};
use log::{info, warn};

// Suffix of the last known-good copy of the wallet database
pub const BACKUP_SUFFIX: &str = ".backup";

fn copy_dir(src: &Path, dst: &Path) -> std::io::Result<()> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let to = dst.join(entry.file_name());
        if entry.file_type()?.is_dir() {
            copy_dir(&entry.path(), &to)?;
        } else {
            fs::copy(entry.path(), &to)?;
        }
    }
    Ok(())
}

// Open a database as-is (used when creating a new wallet)
pub fn open(name: &str) -> anyhow::Result<Db> {
    Ok(sled::open(name)?)
}

// Why an existing wallet database is not usable, or None if it is fine
fn check(db: &sled::Result<Db>, required_key: &[u8]) -> Option<String> {
    match db {
        Err(e) => Some(format!("cannot be opened: {}", e)),
        Ok(db) => match db.contains_key(required_key) {
            Ok(true) => None,
            Ok(false) => Some("opened but its wallet data is missing".to_string()),
            Err(e) => Some(format!("cannot be read: {}", e)),
        },
    }
}

// Open an existing wallet database. If it is damaged or its data is missing
// (for example the app was killed in the middle of a write), keep the damaged
// copy aside and restore the last verified backup.
pub fn open_checked(name: &str, required_key: &[u8]) -> anyhow::Result<Db> {
    let first = sled::open(name);
    let problem = match check(&first, required_key) {
        None => return Ok(first?),
        Some(p) => p,
    };
    drop(first);

    let backup = format!("{}{}", name, BACKUP_SUFFIX);
    if !Path::new(&backup).is_dir() {
        return Err(anyhow::anyhow!(
            "The wallet database {} and no backup copy was found. Please restore your wallet from its seed phrase.", problem
        ));
    }
    let ts = SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0);
    let broken = format!("{}.broken-{}", name, ts);
    warn!("Wallet database {}; keeping it as {} and restoring the last backup", problem, broken);
    if Path::new(name).exists() {
        fs::rename(name, &broken)?;
    }
    copy_dir(Path::new(&backup), Path::new(name))?;

    let restored = sled::open(name);
    if let Some(p) = check(&restored, required_key) {
        return Err(anyhow::anyhow!(
            "The wallet backup {} too. Please restore your wallet from its seed phrase.", p
        ));
    }
    info!("Wallet database restored from backup");
    Ok(restored?)
}

// Make a verified backup copy of the wallet database.
// The copy is test-opened before it replaces the previous backup.
pub fn backup(db: &Db, name: &str) -> anyhow::Result<()> {
    db.flush()?;
    let backup = format!("{}{}", name, BACKUP_SUFFIX);
    let tmp = format!("{}.backup-tmp", name);
    let _ = fs::remove_dir_all(&tmp);
    copy_dir(Path::new(name), Path::new(&tmp))?;
    {
        let test = sled::open(&tmp)?;
        drop(test);
    }
    let _ = fs::remove_dir_all(&backup);
    fs::rename(&tmp, &backup)?;
    Ok(())
}

#[cfg(test)]
mod recovery_tests {
    use super::*;

    fn temp_wallet(tag: &str) -> (std::path::PathBuf, String) {
        let dir = std::env::temp_dir().join(format!("dapa_wallet_{}_{}", tag, std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        let name = dir.join("wallet").to_string_lossy().to_string();
        (dir, name)
    }

    #[test]
    fn restores_from_backup_when_data_lost() {
        let (dir, name) = temp_wallet("recovery");
        {
            let db = open(&name).unwrap();
            db.insert(b"key", b"value").unwrap();
            backup(&db, &name).unwrap();
        }
        // Simulate the database coming back empty after an interrupted write
        fs::remove_dir_all(&name).unwrap();
        fs::create_dir_all(&name).unwrap();

        let db = open_checked(&name, b"key").unwrap();
        assert_eq!(db.get(b"key").unwrap().unwrap().as_ref(), b"value");
        let kept_broken = fs::read_dir(&dir).unwrap()
            .any(|e| e.unwrap().file_name().to_string_lossy().contains(".broken-"));
        assert!(kept_broken, "damaged copy should be kept aside, not deleted");
        drop(db);
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn healthy_wallet_opens_untouched() {
        let (dir, name) = temp_wallet("healthy");
        {
            let db = open(&name).unwrap();
            db.insert(b"key", b"value").unwrap();
            db.flush().unwrap();
        }
        let db = open_checked(&name, b"key").unwrap();
        assert_eq!(db.get(b"key").unwrap().unwrap().as_ref(), b"value");
        let kept_broken = fs::read_dir(&dir).unwrap()
            .any(|e| e.unwrap().file_name().to_string_lossy().contains(".broken-"));
        assert!(!kept_broken, "a healthy wallet must not be touched");
        drop(db);
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn clear_error_without_backup() {
        let (dir, name) = temp_wallet("nobackup");
        fs::create_dir_all(&name).unwrap();
        let err = open_checked(&name, b"key").unwrap_err().to_string();
        assert!(err.contains("seed phrase"), "error should tell the user to restore from seed: {}", err);
        let _ = fs::remove_dir_all(&dir);
    }
}
