use crate::prelude::*;

pub fn say(cmd: cli::Say) -> Result<()> {
    let mut socket = util::get_socket()?;
    socket.send(Message::Say { thing: cmd.thing })?;

    Ok(())
}

