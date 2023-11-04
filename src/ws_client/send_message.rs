pub struct SendMessage {
    msg_type: String,
    id: i64,
    msg: String,
}

impl SendMessage {
    pub fn new(msg_type: &str, id: i64, msg: &str) -> SendMessage {
        let msg_type = msg_type.parse().unwrap();
        let id = id;
        let msg = msg.parse().unwrap();
        return SendMessage { msg_type, id, msg };
    }

    pub fn unpack(&self) -> (&str, i64, &str) {
        return (&self.msg_type, self.id, &self.msg);
    }
}
