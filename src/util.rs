use crate::prelude::*;
use anyhow::anyhow;
use std::process::Command;

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
