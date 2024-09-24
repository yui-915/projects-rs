use crate::prelude::*;

pub fn get_socket() -> Result<Socket<Message, UnixStream>> {
    let stream = UnixStream::connect(SOCKET_PATH)?;
    Ok(Socket::new(stream))
}
