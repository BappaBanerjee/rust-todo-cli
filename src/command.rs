use clap::{Args, Parser, Subcommand};

use crate::Status;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Commands {
    #[command(subcommand)]
    pub command: SubCommands,
}

#[derive(Debug, Subcommand)]
pub enum SubCommands {
    Create(CreateArgs),
    List,
    Complete(CompleteArgs),
    Edit(EditArgs),
    Delete(DeleteArgs),
    Exit,
}

#[derive(Debug, Args)]
pub struct CreateArgs {
    pub name: String,
}

#[derive(Debug, Args)]
pub struct EditArgs {
    pub id: usize,
    #[arg(short, long)]
    pub name: Option<String>,
    #[arg(short, long)]
    pub status: Option<Status>,
}

#[derive(Debug, Args)]
pub struct CompleteArgs {
    pub id: usize,
}

#[derive(Debug, Args)]
pub struct DeleteArgs {
    pub id: usize,
}
