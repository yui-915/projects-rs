use crate::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
pub enum Message {
    Empty,
    StopDaemon { force: bool },
    DaemonStatusRequest,
    DaemonStatusResponse { config_dir: PathBuf },
}
