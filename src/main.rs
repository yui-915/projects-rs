pub mod cli;
mod commands;
mod daemon;
mod protocol;
mod socket;
pub mod util;

use crate::prelude::*;
use cli::{Commands, DaemonCommands};

mod prelude {
    pub use crate::{cli, protocol::Message, socket::Socket, util, SOCKET_PATH};
    pub use anyhow::Result;
    pub use serde::{de::DeserializeOwned, Deserialize, Serialize};
    pub use std::{
        fs,
        io::{Read, Write},
        marker::PhantomData,
        os::unix::net::{UnixListener, UnixStream},
    };
}

pub const SOCKET_PATH: &str = "/tmp/projects-rs-daemon-socket";

fn main() -> Result<()> {
    let cli = cli::parse();
    match cli.command {
        Commands::Daemon(daemon) => match daemon.command {
            DaemonCommands::StartMain => daemon::main()?,
            _ => commands::daemon(daemon)?,
        },
    };

    Ok(())
}

#[cfg(not(target_os = "linux"))]
compile_error!("non-linux user detected!!!!! 0_0");
