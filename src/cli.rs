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
}

#[derive(Parser, Debug)]
pub struct Daemon {
    #[clap(subcommand)]
    pub command: DaemonCommands,
}

#[derive(Subcommand, Debug)]
pub enum DaemonCommands {
    Start,
    Status,
    Kill,
    Stop {
        #[clap(short, long)]
        force: bool,
    },

    #[clap(hide = true)]
    StartMain,
}
