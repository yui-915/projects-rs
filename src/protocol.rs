use crate::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
pub enum Message {
    Empty,
    StopDaemon { force: bool },
}
