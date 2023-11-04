use crate::config::Config;
use crate::ws_client::message_factory::MessageFactory;
use crate::ws_client::send_message::SendMessage;
use futures_util::{SinkExt, StreamExt};
use hyper::header::{HeaderValue, AUTHORIZATION};
use serde_json::json;
use tokio::net::TcpStream;
use tokio_tungstenite::{
    connect_async,
    tungstenite::{client::IntoClientRequest, Message},
    MaybeTlsStream, WebSocketStream,
};

pub struct WsClient {
    pub socket: WebSocketStream<MaybeTlsStream<TcpStream>>,
    pub message_factory: MessageFactory,
}

impl WsClient {
    pub async fn new(config: Config) -> WsClient {
        let mut request = config.address.into_client_request().unwrap();
        if config.auth_token != "" {
            let auth = HeaderValue::from_str(&format!("Bearer {}", config.auth_token)).unwrap();
            let headers = request.headers_mut();
            headers.insert(AUTHORIZATION, auth);
        }
        let (socket, _) = connect_async(request).await.expect("Failed to connect");
        return WsClient {
            socket,
            message_factory: MessageFactory {},
        };
    }

    pub async fn receive_msg(mut self) {
        while let Some(msg) = self.socket.next().await {
            let receive_time = chrono::Local::now().timestamp_nanos_opt().unwrap();
            match msg {
                Ok(msg) => {
                    if let Message::Text(text) = msg {
                        println!("{text}");
                        match self.message_factory.check(&text, receive_time).await {
                            Some(msg) => self.send_msg(msg).await,
                            None => {}
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error: {:?}", e);
                    break;
                }
            }
        }
    }

    pub async fn send_msg(&mut self, message: SendMessage) {
        let (msg_type, id, msg) = message.unpack();
        let mut msg_object = json!(
        {
            "action": "send_msg",
            "params": {
                "message_type": "private",
                "user_id": -1,
                "group_id": -1,
                "message": ""
            }
        }
        );
        let param = msg_object.get_mut("params").unwrap();
        let message_type = param.get_mut("message_type").unwrap();
        *message_type = msg_type.into();

        if msg_type == "private" {
            let user_id = param.get_mut("user_id").unwrap();
            *user_id = id.into();
        } else if msg_type == "group" {
            let group_id = param.get_mut("group_id").unwrap();
            *group_id = id.into();
        }

        let message = param.get_mut("message").unwrap();
        *message = msg.into();

        println!("{msg_object}");

        self.socket
            .send(Message::Text(msg_object.to_string()))
            .await
            .expect("TODO: panic message");
    }
}
