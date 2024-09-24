use crate::prelude::*;

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

fn handle_connection(mut stream: Socket<Message, UnixStream>) -> Result<()> {
    loop {
        let msg = stream.recv()?;
        println!("Received message: {:?}", msg);
    }
}

