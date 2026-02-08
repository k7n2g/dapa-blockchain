mod rpc_server;
mod xswd_server;

use serde::Serialize;
use serde_json::json;
use dapa_common::{
    api::wallet::NotifyEvent,
    rpc::{ShareableTid, server::WebSocketServerHandler}
};
use crate::api::XSWDHandler;
pub use xswd_server::{
    XSWDServer,
    XSWDWebSocketHandler
};
pub use rpc_server::{
    WalletRpcServer,
    WalletRpcServerShared,
    AuthConfig
};

pub enum APIServer<W>
where
    W: ShareableTid<'static> + XSWDHandler
{
    RPCServer(WalletRpcServerShared<W>),
    XSWD(XSWDServer<W>)
}

impl<W> APIServer<W>
where
    W: ShareableTid<'static> + XSWDHandler
{
    pub async fn notify_event<V: Serialize>(&self, event: &NotifyEvent, value: &V) {
        let json = json!(value);
        match self {
            APIServer::RPCServer(server) => {
                server.get_websocket().get_handler().notify(event, json).await;
            },
            APIServer::XSWD(xswd) => {
                xswd.get_handler().notify(event, json).await;
            }
        }
    }

    pub async fn stop(self) {
        match self {
            APIServer::RPCServer(server) => {
                server.stop().await;
            },
            APIServer::XSWD(xswd) => {
                xswd.stop().await;
            }
        }
    }
}