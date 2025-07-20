use clap::{Parser, Subcommand, Args};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Commands {
    #[command(subcommand)]
    pub command: SubCommands,
}

#[derive(Debug, Subcommand)]
pub enum SubCommands {
    Create(CreateArgs),
    List
}

#[derive(Debug, Args)]
pub struct CreateArgs{
    pub name : String,
    // completed : bool
}

