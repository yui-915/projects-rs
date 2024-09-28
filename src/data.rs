use crate::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Project {
    pub name: String,
    pub path: PathBuf,
    pub command: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Message {
    Empty,
    Poll,
    StopDaemon { force: bool },
    DaemonStatusRequest,
    DaemonStatusResponse { config_dir: PathBuf },
    NewProjectRequest { project: Project },
    NewProjectResponse,
    ListProjectsRequest,
    ListProjectsResponse { projects: Vec<String> },
    StartProjectRequest { project: String },
    StartProjectResponse { success: bool },
    DeleteProjectRequest { project: String },
    DeleteProjectResponse { success: bool },
    AttachProjectRequest { project: String },
    AttachProjectResponse { success: bool },
    AttachData { data: Vec<u8> },
    Detach,
    StopProjectRequest { project: String },
    StopProjectResponse { success: bool },
    KillProjectRequest { project: String },
    KillProjectResponse { success: bool },
}
