use crate::prelude::*;

pub fn stop(stop: cli::Stop) -> Result<()> {
    let mut socket = util::get_socket()?;
    socket.send(Message::StopProjectRequest {
        project: stop.project,
    })?;

    let msg = socket.recv()?;
    match msg {
        Message::StopProjectResponse { success } => {
            if success {
                println!("stopped?");
            } else {
                println!("failed to stop");
            }
        }
        _ => Err(anyhow!("unknown message"))?,
    }

    Ok(())
}
