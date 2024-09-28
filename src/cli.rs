use crate::prelude::*;
use clap::{Parser, Subcommand};

pub fn parse() -> Cli {
    Cli::parse()
}

#[derive(Parser, Debug)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Daemon(Daemon),
    New(New),
    List(List),
    Delete(Delete),
    Start(Start),
    Attach(Attach),
    Stop(Stop),
    Kill(Kill),
}

#[derive(Parser, Debug)]
pub struct Daemon {
    #[clap(subcommand)]
    pub command: DaemonCommands,
}

#[derive(Subcommand, Debug)]
pub enum DaemonCommands {
    Start {
        #[clap(short, long)]
        configs_dir: Option<PathBuf>,
    },
    Status,
    Kill,
    Stop {
        #[clap(short, long)]
        force: bool,
    },

    #[clap(hide = true)]
    StartMain {
        configs_dir: PathBuf,
    },
}

#[derive(Parser, Debug)]
pub struct New {}

#[derive(Parser, Debug)]
pub struct List {}

#[derive(Parser, Debug)]
pub struct Delete {
    pub project: String,
}

#[derive(Parser, Debug)]
pub struct Start {
    pub project: String,
}

#[derive(Parser, Debug)]
pub struct Attach {
    pub project: String,
}

#[derive(Parser, Debug)]
pub struct Stop {
    pub project: String,
}

#[derive(Parser, Debug)]
pub struct Kill {
    pub project: String,
}
