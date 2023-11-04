use crate::ws_client::send_message::SendMessage;
use rand::{thread_rng, Rng};
use serde_json::{from_str, Value};

pub struct MessageFactory {}
impl MessageFactory {
    pub async fn check(&mut self, text: &str, receive_time: i64) -> Option<SendMessage> {
        let json: Value = from_str(text).unwrap();

        match json.get("post_type") {
            Some(post_type) => {
                if post_type != "message" {
                    return None;
                }
            }
            None => return None,
        }

        let raw_message = json.get("raw_message").unwrap().as_str()?;
        let split: Vec<&str> = raw_message.split_whitespace().collect();
        let mut msg = String::new();

        let command = *split.get(0)?;
        match command {
            "/ping" => {
                msg = self.get_ping(receive_time).await;
            }

            "/roll" => {
                if split.len() == 1 {
                    msg = self.get_roll(-1).await;
                } else if split.len() == 2 {
                    match split.get(1)?.parse::<i64>() {
                        Ok(num) => msg = self.get_roll(num).await,
                        Err(_) => msg = self.get_roll(-1).await,
                    }
                } else if split.len() > 2 {
                    msg = self.get_roll_vec(split[1..split.len()].to_vec()).await;
                }
            }

            _ => return None,
        }

        return Some(self.pack_message(json, msg).await);
    }

    async fn get_ping(&self, receive_time: i64) -> String {
        let ping = chrono::Local::now().timestamp_nanos_opt().unwrap() - receive_time;
        return format!("本次的逻辑延迟为：{:.4} ms", ping as f64 * 0.0000001);
    }

    async fn get_roll(&self, num: i64) -> String {
        return if num < 1 {
            format!("生成 [0-9] 随机值：{}", thread_rng().gen_range(0..10))
        } else {
            format!(
                "生成 [0-{}] 随机值：{}",
                num,
                thread_rng().gen_range(0..num)
            )
        };
    }

    async fn get_roll_vec(&self, contains: Vec<&str>) -> String {
        let num: usize = thread_rng().gen_range(0..contains.len());
        return format!("随机结果为：{:?}", contains.get(num).unwrap());
    }

    async fn pack_message(&self, json: Value, msg: String) -> SendMessage {
        let message_type = json.get("message_type").unwrap().as_str().unwrap();
        return match json.get("group_id") {
            None => SendMessage::new(
                message_type,
                json.get("user_id").unwrap().as_i64().unwrap(),
                &msg,
            ),
            Some(_id) => SendMessage::new(message_type, _id.as_i64().unwrap(), &msg),
        };
    }
}
