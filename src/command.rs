use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum FromClientMessage {
    Chat(String),
    Command(String),
}

#[derive(Serialize, Deserialize)]
pub struct FromServerMessage {
    message_type: u8,
    input_data: String,
}
