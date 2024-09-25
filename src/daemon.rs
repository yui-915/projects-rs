use crate::prelude::*;
use anyhow::anyhow;

type Sock = Socket<Message, UnixStream>;

pub fn main() -> Result<()> {
    if fs::exists(SOCKET_PATH)? {
        fs::remove_file(SOCKET_PATH)?;
    }

    let listener = UnixListener::bind(SOCKET_PATH)?;
    println!("Listening on {:?}", SOCKET_PATH);

    while let Ok((stream, _)) = listener.accept() {
        println!("New connection");
        match handle_connection(Socket::new(stream)) {
            Ok(_) => println!("Connection closed"),
            Err(e) => println!("Connection closed: {}", e),
        }
    }

    Ok(())
}

fn handle_connection(mut socket: Sock) -> Result<()> {
    loop {
        let msg = socket.recv()?;
        match msg {
            Message::StopDaemon { force } => stop_daemon(&mut socket, force)?,
            _ => Err(anyhow!("unknown message"))?,
        }
    }
}

fn stop_daemon(socket: &mut Sock, _force: bool) -> Result<()> {
    socket.send(Message::Empty)?;
    println!("stopping daemon ...");
    std::process::exit(0);
}
