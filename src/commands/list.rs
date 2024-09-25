use crate::prelude::*;

pub fn list(_: cli::List) -> Result<()> {
    if !util::is_daemon_running()? {
        println!("daemon is not running");
        return Ok(());
    }

    let mut socket = util::get_socket()?;
    socket.send(Message::ListProjectsRequest)?;

    let msg = socket.recv()?;
    match msg {
        Message::ListProjectsResponse { projects } => {
            for (i, project) in projects.iter().enumerate() {
                println!("{}. {}", i + 1, project);
            }
        }
        _ => Err(anyhow!("unknown message"))?,
    }
    Ok(())
}
