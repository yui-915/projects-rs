use crate::prelude::*;

pub fn kill(kill: cli::Kill) -> Result<()> {
    let mut socket = util::get_socket()?;
    socket.send(Message::KillProjectRequest {
        project: kill.project,
    })?;

    let msg = socket.recv()?;
    match msg {
        Message::KillProjectResponse { success } => {
            if success {
                println!("killed (probably)");
            } else {
                println!("not killed?!");
            }
        }
        _ => Err(anyhow!("unknown message"))?,
    }

    Ok(())
}
