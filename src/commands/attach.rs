use crate::prelude::*;
use termion::raw::IntoRawMode;
// use termion::screen::IntoAlternateScreen;

pub fn attach(args: cli::Attach) -> Result<()> {
    let mut socket = util::get_socket()?;
    socket.send(Message::AttachProjectRequest {
        project: args.project,
    })?;

    let msg = socket.recv()?;
    match msg {
        Message::AttachProjectResponse { success } => {
            if success {
                main_attach_loop(socket)?;
            } else {
                println!("failed to attach");
            }
        }
        _ => Err(anyhow!("unknown message"))?,
    };

    Ok(())
}

fn main_attach_loop(mut socket: Socket<Message, UnixStream>) -> Result<()> {
    // let mut stdout = std::io::stdout().into_raw_mode()?.into_alternate_screen()?;
    let mut stdout = std::io::stdout().into_raw_mode()?;
    // write!(
    //     stdout,
    //     "{}{}",
    //     termion::clear::All,
    //     termion::cursor::Goto(1, 1)
    // )?;
    std::process::Command::new("clear").status()?;

    let stdin = util::channel_thread(std::io::stdin);

    loop {
        let msg = socket.recv()?;
        match msg {
            Message::Poll => {}

            Message::Detach => {
                return Ok(());
            }

            Message::AttachData { data } => {
                stdout.write_all(&data)?;
                stdout.flush()?;
            }

            _ => Err(anyhow!("unknown message"))?,
        }
        if let Ok(data) = stdin.try_recv() {
            if let Some(idx) = data.iter().position(|&b| b == 3) {
                socket.send(Message::AttachData {
                    data: data[..idx].to_vec(),
                })?;
                socket.send(Message::Detach)?;
                std::process::Command::new("clear").status()?;
                return Ok(());
            } else {
                socket.send(Message::AttachData { data })?;
            }
        } else {
            util::sleep(10);
            socket.send(Message::Poll)?;
        }
    }
}
