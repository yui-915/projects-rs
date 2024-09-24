use crate::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
pub enum Message {
    Say { thing: String },
}
