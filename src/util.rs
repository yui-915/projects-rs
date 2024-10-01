use crate::prelude::*;
use std::process::Command;
use std::sync::mpsc;

pub fn get_exe_name() -> Result<String> {
    let exe = std::env::current_exe()?;
    let name = exe.file_name().ok_or(anyhow!("no file name"))?;
    let name_as_str = name.to_str().ok_or(anyhow!("cannot convert to str"))?;
    Ok(name_as_str.to_string())
}

pub fn get_socket() -> Result<Socket<Message, UnixStream>> {
    let stream = UnixStream::connect(SOCKET_PATH)?;
    Ok(Socket::new(stream))
}

pub fn cmd(cmd: &[&str]) -> Result<String> {
    let output = Command::new(cmd[0]).args(cmd.iter().skip(1)).output()?;
    let res = String::from_utf8_lossy(&output.stdout).to_string();
    Ok(res)
}

pub fn get_other_selves_pids() -> Result<Vec<String>> {
    let my_pid = std::process::id().to_string();
    let my_name = util::get_exe_name()?;
    let pids_str = util::cmd(&["pidof", &my_name, "-o", &my_pid])?;
    Ok(pids_str.split_whitespace().map(|s| s.to_string()).collect())
}

pub fn is_daemon_running() -> Result<bool> {
    let pids = get_other_selves_pids()?;
    Ok(!pids.is_empty())
}

pub fn sleep(millis: u64) {
    let duration = std::time::Duration::from_millis(millis);
    std::thread::sleep(duration);
}

pub fn channel_thread<T: Read, F: Send + 'static + FnOnce() -> T>(f: F) -> mpsc::Receiver<Vec<u8>> {
    let (tx, rx) = mpsc::channel::<Vec<u8>>();
    std::thread::spawn(move || {
        let mut stream = f();
        loop {
            let mut buffer = [0; 1048576]; // 1MiB
            let n = stream.read(&mut buffer).unwrap();
            if n > 0 {
                tx.send(buffer[..n].to_vec()).unwrap();
            }
            sleep(10);
        }
    });
    rx
}

#[macro_export]
macro_rules! map_enum {
    {
        match $thing:expr;
        $enum:ident::... => $mod:ident::...;
        $($variant:ident => $method:ident;)*
        $(
            !end auto;
            $($tt:tt)+
        )?
    } => {
        match $thing {
        $(
            $enum::$variant(i) => $mod::$method(i),
        )*
        $($($tt)+)?
        }
    };
}
