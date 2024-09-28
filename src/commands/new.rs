use crate::prelude::*;

pub fn new(args: cli::New) -> Result<()> {
    if !util::is_daemon_running()? {
        println!("daemon is not running");
        return Ok(());
    }

    let mut name = args.name;
    let mut path = args.path;
    let mut command = args.command;

    while name.is_none() {
        print!("Project name: ");
        std::io::stdout().flush()?;

        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        input = input.trim().to_string();

        if input.is_empty() {
            continue;
        }
        name = Some(input);
    }

    while path.is_none() {
        print!("Project path (default: current dir): ");
        std::io::stdout().flush()?;

        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        input = input.trim().to_string();

        if input.is_empty() {
            input = ".".to_string();
        }
        let input = PathBuf::from(input);
        if !input.exists() {
            println!("path does not exist");
            continue;
        }

        let input = fs::canonicalize(input)?;
        path = Some(input.to_path_buf());
    }

    while command.is_none() {
        print!("Command: ");
        std::io::stdout().flush()?;

        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        input = input.trim().to_string();

        if input.is_empty() {
            continue;
        }
        command = Some(input);
    }

    let project = Project {
        name: name.unwrap(),
        path: path.unwrap(),
        command: command.unwrap(),
    };
    let mut socket = util::get_socket()?;
    socket.send(Message::NewProjectRequest { project })?;
    socket.recv()?;
    println!("New project created");
    Ok(())
}
