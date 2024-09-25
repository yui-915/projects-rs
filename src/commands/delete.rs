use crate::prelude::*;

pub fn delete(delete: cli::Delete) -> Result<()> {
    if !util::is_daemon_running()? {
        println!("daemon is not running");
        return Ok(());
    }

    let mut socket = util::get_socket()?;
    socket.send(Message::DeleteProjectRequest {
        project: delete.project,
    })?;

    let res = socket.recv()?;
    match res {
        Message::DeleteProjectResponse { success } => {
            if success {
                println!("Project deleted");
            } else {
                println!("Project does not exist?");
            }
        }
        _ => Err(anyhow!("unknown message"))?,
    }

    Ok(())
}
