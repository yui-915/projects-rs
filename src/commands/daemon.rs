use crate::prelude::*;
use std::{
    os::unix::process::CommandExt,
    process::{Command, Stdio},
};

pub fn daemon(cmd: cli::Daemon) -> Result<()> {
    match cmd.command {
        cli::DaemonCommands::Start { configs_dir } => start_daemon(configs_dir),
        cli::DaemonCommands::Status => daemon_status(),
        cli::DaemonCommands::Kill => kill_daemon(),
        cli::DaemonCommands::Stop { force } => stop_daemon(force),
        cli::DaemonCommands::StartMain { .. } => unreachable!(),
    }
}

fn start_daemon(mut configs_dir: Option<PathBuf>) -> Result<()> {
    if util::is_daemon_running()? {
        println!("daemon is already running");
        return Ok(());
    }

    if configs_dir.is_none() {
        let home = std::env::var("HOME");
        if let Ok(home) = home {
            configs_dir = Some(PathBuf::from(home).join(".projects-rs"));
        } else {
            return Err(anyhow!("cannot find home, please specify configs dir"));
        }
    }

    let configs_dir = configs_dir.unwrap();
    fs::create_dir_all(&configs_dir)?;

    let configs_dir = fs::canonicalize(configs_dir)?;
    let configs_dir = configs_dir
        .to_str()
        .ok_or(anyhow!("cannot convert to str"))?;

    Command::new(std::env::current_exe()?)
        .args(["daemon", "start-main", configs_dir])
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .process_group(0)
        .spawn()?;

    println!("daemon started");
    Ok(())
}

fn daemon_status() -> Result<()> {
    if !util::is_daemon_running()? {
        println!("daemon is not running");
        return Ok(());
    }

    println!("daemon is running");
    let mut socket = util::get_socket()?;
    socket.send(Message::DaemonStatusRequest)?;

    let msg = socket.recv()?;
    match msg {
        Message::DaemonStatusResponse { config_dir } => {
            println!("configs dir: {}", config_dir.display());
        }
        _ => Err(anyhow!("unknown message"))?,
    }

    Ok(())
}

fn kill_daemon() -> Result<()> {
    let pids = util::get_other_selves_pids()?;

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
    if !util::is_daemon_running()? {
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
