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
    Say(Say),
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
    Stop,

    #[clap(hide = true)]
    StartMain,
}

#[derive(Parser, Debug)]
pub struct Say {
    pub thing: String,
}
