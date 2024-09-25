use crate::prelude::*;
use std::{
    os::unix::process::CommandExt,
    process::{Command, Stdio},
};

pub fn daemon(cmd: cli::Daemon) -> Result<()> {
    match cmd.command {
        cli::DaemonCommands::Start => start_daemon(),
        cli::DaemonCommands::Status => daemon_status(),
        cli::DaemonCommands::Kill => kill_daemon(),
        cli::DaemonCommands::Stop { force } => stop_daemon(force),
        cli::DaemonCommands::StartMain => unreachable!(),
    }
}

fn start_daemon() -> Result<()> {
    if is_daemon_running()? {
        println!("daemon is already running");
        return Ok(());
    }

    Command::new(std::env::current_exe()?)
        .args(["daemon", "start-main"])
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .process_group(0)
        .spawn()?;

    println!("daemon started");
    Ok(())
}

fn daemon_status() -> Result<()> {
    if is_daemon_running()? {
        println!("daemon is running");
    } else {
        println!("daemon is not running");
    }

    Ok(())
}

fn kill_daemon() -> Result<()> {
    let pids = get_other_selves_pids()?;

    if pids.is_empty() {
        println!("daemon is not running");
        return Ok(());
    }

    for pid in pids {
        util::cmd(&["kill", "-9", &pid])?;
    }

    println!("daemon killed!");

    Ok(())
}

fn stop_daemon(force: bool) -> Result<()> {
    if !is_daemon_running()? {
        println!("daemon is not running");
        return Ok(());
    }

    let mut socket = util::get_socket()?;
    socket.send(Message::StopDaemon { force })?;
    println!("stopping daemon ...");

    socket.recv()?;
    println!("daemon stopped?");

    Ok(())
}

fn get_other_selves_pids() -> Result<Vec<String>> {
    let my_pid = std::process::id().to_string();
    let my_name = util::get_exe_name()?;
    let pids_str = util::cmd(&["pidof", &my_name, "-o", &my_pid])?;
    Ok(pids_str.split_whitespace().map(|s| s.to_string()).collect())
}

fn is_daemon_running() -> Result<bool> {
    let pids = get_other_selves_pids()?;
    Ok(!pids.is_empty())
}
