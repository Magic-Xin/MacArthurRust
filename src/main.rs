mod config;
mod ws_client;

use crate::config::read_from_file;
use crate::ws_client::ws_client::WsClient;

#[tokio::main]
async fn main() {
    let cfg = read_from_file("config.jsonc").await.unwrap();
    let wsc = WsClient::new(cfg).await;
    wsc.receive_msg().await;
}
