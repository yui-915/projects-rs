use crate::prelude::*;

pub fn start(start: cli::Start) -> Result<()> {
    let mut socket = util::get_socket()?;
    socket.send(Message::StartProjectRequest {
        project: start.project,
    })?;

    let msg = socket.recv()?;
    match msg {
        Message::StartProjectResponse { success } => {
            if success {
                println!("started");
            } else {
                println!("failed to start");
            }
        }
        _ => Err(anyhow!("unknown message"))?,
    }

    Ok(())
}
