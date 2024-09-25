use crate::prelude::*;

type Sock = Socket<Message, UnixStream>;

static mut CONFIGS_DIR: Option<PathBuf> = None;
fn configs_dir() -> &'static PathBuf {
    unsafe { CONFIGS_DIR.as_ref().unwrap() }
}

pub fn main(configs_dir: PathBuf) -> Result<()> {
    unsafe {
        CONFIGS_DIR = Some(configs_dir);
    }

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
            Message::DaemonStatusRequest => send_daemon_status(&mut socket)?,
            _ => Err(anyhow!("unknown message"))?,
        }
    }
}

fn stop_daemon(socket: &mut Sock, _force: bool) -> Result<()> {
    socket.send(Message::Empty)?;
    println!("stopping daemon ...");
    std::process::exit(0);
}

fn send_daemon_status(socket: &mut Sock) -> Result<()> {
    let config_dir = configs_dir().to_path_buf();
    socket.send(Message::DaemonStatusResponse { config_dir })?;
    Ok(())
}
