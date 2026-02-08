use std::{borrow::Cow, collections::HashMap, sync::Arc};

use futures::{SinkExt, StreamExt};
use log::{error, debug};
use serde_json::json;
use tokio_tungstenite_wasm::{
    WebSocketStream,
    Message,
    connect,
};
use dapa_common::{
    api::daemon::NotifyEvent,
    rpc::{RpcResponse, ShareableTid},
    tokio::{
        select,
        spawn_task,
        sync::{RwLock, mpsc},
        task
    }
};
use crate::api::{
    xswd::relayer::{
        cipher::Cipher,
        XSWDRelayerShared
    },
    AppStateShared,
    EncryptionMode,
    XSWDHandler,
    XSWDResponse,
};

enum InternalMessage {
    Send(String),
    Close,
}

pub struct ClientImpl {
    target: String,
    sender: mpsc::Sender<InternalMessage>,
    events: RwLock<HashMap<NotifyEvent, task::JoinHandle<()>>>,
}

pub type Client = Arc<ClientImpl>;

impl ClientImpl {
    pub async fn new<W>(target: String, relayer: XSWDRelayerShared<W>, encryption_mode: Option<EncryptionMode>, state: AppStateShared) -> Result<Client, anyhow::Error>
    where
        W: ShareableTid<'static> + XSWDHandler
    {
        // Create a cipher based on the provided encryption mode
        let cipher = Cipher::new(encryption_mode)?;

        let ws = connect(&target).await?;
        let (sender, receiver) = mpsc::channel(64);

        let client = Arc::new(Self {
            target,
            sender,
            events: RwLock::new(HashMap::new()),
        });

        {
            let client = client.clone();
            spawn_task(format!("xswd-relayer-{}", state.get_id()), async move {
                if let Err(e) = Self::background_task(client, ws, &state, &relayer, receiver, cipher).await {
                    debug!("Error on xswd relayer #{}: {}", state.get_id(), e);
                }
    
                relayer.on_close(state).await;
            });
        }

        Ok(client)
    }

    pub fn target(&self) -> &str {
        &self.target
    }

    pub async fn send_message<V: ToString>(&self, msg: V) -> bool {
        if let Err(e) = self.sender.send(InternalMessage::Send(msg.to_string())).await {
            error!("Error while sending message: {}", e);
            return false;
        }

        true
    }

    pub async fn close(&self) {
        if let Err(e) = self.sender.send(InternalMessage::Close).await {
            error!("Error while sending close message: {}", e);
        }
    }

    async fn background_task<W>(
        client: Client,
        mut ws: WebSocketStream,
        state: &AppStateShared,
        relayer: &XSWDRelayerShared<W>,
        mut receiver: mpsc::Receiver<InternalMessage>,
        mut cipher: Cipher
    ) -> Result<(), anyhow::Error>
    where
        W: ShareableTid<'static> + XSWDHandler
    {
        loop {
            select! {
                msg = ws.next() => {
                    let Some(Ok(msg)) = msg else {
                        break;
                    };

                    let bytes: &[u8] = match &msg {
                        Message::Text(bytes) => bytes.as_ref(),
                        Message::Binary(bytes) => &bytes,
                        Message::Close(_) => {
                            break;
                        }
                    };

                    let output = cipher.decrypt(bytes)?;
                    let response = match relayer.on_message(state, &output).await {
                        Ok(response) => match response {
                            XSWDResponse::Request(value) => match value {
                                Some(v) => v,
                                None => continue,
                            },
                            XSWDResponse::Event(event, stream, value) => {
                                let mut lock = client.events.write().await;

                                match stream {
                                    Some((mut stream, id)) => {
                                        if !lock.contains_key(&event) {
                                            // spawn a task to handle the event stream
                                            let client = client.clone();
                                            let handle = spawn_task("xswd-relayer-event-listener", async move {
                                                while let Ok(value) = stream.recv().await {
                                                    let response = json!(RpcResponse::new(Cow::Borrowed(&id), Cow::Borrowed(&value)));
                                                    if !client.send_message(response.to_string()).await {
                                                        break;
                                                    }
                                                }
                                            });

                                            lock.insert(event, handle);
                                        }
                                    },
                                    None => {
                                        if let Some(handle) = lock.remove(&event) {
                                            handle.abort();
                                        }
                                    }
                                }

                                match value {
                                    Some(v) => v,
                                    None => continue,
                                }
                            },
                        },
                        Err(e) => e.to_json()
                    };

                    // Encrypt response before sending
                    let encrypted_response = cipher.encrypt(response.to_string().as_bytes())?
                        .into_owned();
                    ws.send(Message::Binary(encrypted_response.into())).await?;
                },
                msg = receiver.recv() => {
                    let Some(msg) = msg else {
                        break;
                    };

                    match msg {
                        InternalMessage::Send(msg) => {
                            let output = cipher.encrypt(msg.as_bytes())?
                                .into_owned();
                            ws.send(Message::Binary(output.into())).await?;
                        },
                        InternalMessage::Close => break,
                    }
                },
                else => break,
            };
        }

        Ok(())
    }
}