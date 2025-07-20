use clap::{Args, Parser, Subcommand};

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
    Delete(DeleteArgs),
    Exit,
    // Help
}

#[derive(Debug, Args)]
pub struct CreateArgs {
    pub name: String,
    // completed : bool
}

#[derive(Debug, Args)]
pub struct CompleteArgs {
    pub id: usize,
}

#[derive(Debug, Args)]
pub struct DeleteArgs {
    pub id: usize,
}
