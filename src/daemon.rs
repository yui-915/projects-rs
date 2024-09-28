use crate::prelude::*;
use std::collections::HashMap;
use std::process::Child;
use std::process::Command;
use std::process::Stdio;

type Sock = Socket<Message, UnixStream>;

struct Proc {
    log_file: PathBuf,
    child: Child,
}

static mut CONFIGS_DIR: Option<PathBuf> = None;
fn configs_dir() -> &'static PathBuf {
    let configs_dir = unsafe { CONFIGS_DIR.as_ref().unwrap() };
    if !configs_dir.exists() {
        fs::create_dir_all(configs_dir).unwrap();
    }
    configs_dir
}

fn logs_dir() -> PathBuf {
    let configs_dir = configs_dir();
    let logs_dir = configs_dir.join("logs");
    if !logs_dir.exists() {
        fs::create_dir_all(&logs_dir).unwrap();
    }
    logs_dir
}

static mut PROCS: Option<HashMap<String, Proc>> = None;
fn procs_mut() -> &'static mut HashMap<String, Proc> {
    if unsafe { PROCS.is_none() } {
        unsafe { PROCS = Some(HashMap::new()) };
    }
    unsafe { PROCS.as_mut().unwrap() }
}

fn clean_procs() {
    let keys = procs_mut().keys().collect::<Vec<_>>();
    for proc_name in keys {
        if let Some(proc) = procs_mut().get_mut(proc_name) {
            if !is_proc_still_alive(proc) {
                procs_mut().remove(proc_name);
            }
        }
    }
}

fn is_proc_still_alive(proc: &mut Proc) -> bool {
    if let Ok(status) = proc.child.try_wait() {
        status.is_none()
    } else {
        false
    }
}

fn date() -> String {
    let output = Command::new("date")
        .arg("--rfc-3339=second")
        .output()
        .unwrap();
    String::from_utf8(output.stdout)
        .unwrap()
        .trim()
        .replace(" ", "_")
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
        let mut socket = Socket::new(stream);
        match handle_connection(&mut socket) {
            Ok(_) => println!("Connection closed"),
            Err(e) => {
                println!("Connection closed: {}", e);
                let _ = socket.send(Message::Empty); // would rather have the client error than hang
            }
        }
    }

    Ok(())
}

fn handle_connection(socket: &mut Sock) -> Result<()> {
    loop {
        let msg = socket.recv()?;
        println!("received: {:?}", msg);
        match msg {
            Message::StopDaemon { force } => stop_daemon(socket, force)?,
            Message::DaemonStatusRequest => send_daemon_status(socket)?,
            Message::NewProjectRequest { project } => new_project(socket, project)?,
            Message::ListProjectsRequest => list_projects(socket)?,
            Message::DeleteProjectRequest { project } => delete_project(socket, project)?,
            Message::StartProjectRequest { project } => start_project(socket, project)?,
            Message::AttachProjectRequest { project } => attach_project(socket, project)?,
            Message::StopProjectRequest { project } => stop_project(socket, project)?,
            Message::KillProjectRequest { project } => kill_project(socket, project)?,
            _ => Err(anyhow!("unknown message"))?,
        }
    }
}

fn stop_daemon(socket: &mut Sock, force: bool) -> Result<()> {
    socket.send(Message::Empty)?;
    println!("stopping daemon ...");
    let keys = procs_mut().keys().collect::<Vec<_>>();
    for proc_name in keys {
        if let Some(proc) = procs_mut().get(proc_name) {
            let pid = proc.child.id();
            let mut cmd = Command::new("kill");
            if force {
                cmd.arg("-9");
            }
            cmd.arg(pid.to_string());
            cmd.status()?;
        }
    }
    std::process::exit(0);
}

fn stop_project(socket: &mut Sock, project_name: String) -> Result<()> {
    clean_procs();
    if let Some(proc) = procs_mut().get(&project_name) {
        let pid = proc.child.id();
        let mut cmd = Command::new("kill");
        cmd.arg(pid.to_string());
        cmd.status()?;
        socket.send(Message::StopProjectResponse { success: true })?;
    } else {
        socket.send(Message::StopProjectResponse { success: false })?;
    }
    Ok(())
}

fn kill_project(socket: &mut Sock, project_name: String) -> Result<()> {
    clean_procs();
    if let Some(proc) = procs_mut().get(&project_name) {
        let pid = proc.child.id();
        let mut cmd = Command::new("kill");
        cmd.arg("-9");
        cmd.arg(pid.to_string());
        cmd.status()?;
        socket.send(Message::StopProjectResponse { success: true })?;
    } else {
        socket.send(Message::StopProjectResponse { success: false })?;
    }
    Ok(())
}

fn send_daemon_status(socket: &mut Sock) -> Result<()> {
    let config_dir = configs_dir().to_path_buf();
    socket.send(Message::DaemonStatusResponse { config_dir })?;
    Ok(())
}

fn new_project(socket: &mut Sock, project: Project) -> Result<()> {
    let serialized = toml::to_string_pretty(&project)?;
    fs::write(configs_dir().join(project.name + ".toml"), serialized)?;
    socket.send(Message::NewProjectResponse)?;
    Ok(())
}

fn list_projects(socket: &mut Sock) -> Result<()> {
    let projects = fs::read_dir(configs_dir())?
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().unwrap().is_file())
        .map(|entry| entry.file_name().into_string().unwrap())
        .map(|name| name[..name.len() - 5].to_string()) // remove .toml
        .collect();
    socket.send(Message::ListProjectsResponse { projects })?;
    Ok(())
}

fn delete_project(socket: &mut Sock, project: String) -> Result<()> {
    let path = configs_dir().join(project + ".toml");
    if !path.exists() {
        socket.send(Message::DeleteProjectResponse { success: false })?;
        return Ok(());
    }

    fs::remove_file(path)?;
    socket.send(Message::DeleteProjectResponse { success: true })?;
    Ok(())
}

fn start_project(socket: &mut Sock, project_name: String) -> Result<()> {
    clean_procs();
    let procs = procs_mut();
    if procs.contains_key(&project_name) {
        socket.send(Message::StartProjectResponse { success: false })?;
        return Ok(());
    }

    let path = configs_dir().join(format!("{}.toml", project_name));
    if !path.exists() {
        socket.send(Message::StartProjectResponse { success: false })?;
        return Ok(());
    }

    let raw_project = fs::read_to_string(path)?;
    let project: Project = toml::from_str(&raw_project)?;

    let date = date();
    let log_file_name = logs_dir().join(format!("{}_{}.log", project.name, date));
    let log_file = fs::File::create(&log_file_name)?;

    let child = Command::new("script")
        .args([
            "-qe",
            "-c",
            &project.command,
            log_file_name.to_str().unwrap(),
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::from(log_file.try_clone()?))
        .stderr(Stdio::from(log_file))
        .current_dir(project.path)
        .spawn()?;

    let proc = Proc {
        log_file: log_file_name,
        child,
    };
    procs_mut().insert(project.name, proc);

    socket.send(Message::StartProjectResponse { success: true })?;

    Ok(())
}

fn attach_project(socket: &mut Sock, project_name: String) -> Result<()> {
    clean_procs();

    let procs = procs_mut();
    if !procs.contains_key(&project_name) {
        socket.send(Message::AttachProjectResponse { success: false })?;
        return Ok(());
    } else {
        socket.send(Message::AttachProjectResponse { success: true })?;
    }

    let proc = procs.get_mut(&project_name).unwrap();

    let file = fs::File::open(&proc.log_file)?;
    let stdout = util::channel_thread(|| file);
    loop {
        if !is_proc_still_alive(proc) {
            procs_mut().remove(&project_name);
            socket.send(Message::Detach)?;
            return Ok(());
        }
        if let Ok(data) = stdout.try_recv() {
            socket.send(Message::AttachData { data })?;
        } else {
            socket.send(Message::Poll)?;
        }
        let msg = socket.recv()?;
        match msg {
            Message::Poll => {}
            Message::AttachData { data } => {
                if let Some(stdin) = &mut proc.child.stdin {
                    stdin.write_all(&data)?;
                    stdin.flush()?;
                }
            }
            _ => Err(anyhow!("unknown message"))?,
        }
    }
}
