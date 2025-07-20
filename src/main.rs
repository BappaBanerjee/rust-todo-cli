use clap::Parser;
//todo cli app
use std::io::{self, Write};
use std::ops::Sub;
use todo_cli::Task;

use todo_cli::command::{Commands, CreateArgs, SubCommands};

use todo_cli::{create, delete_task, mark_complete, view_todolist, help};
fn main() {
    let mut todo_lists: Vec<Task> = Vec::new();

    loop {
        io::stdout().flush().unwrap();

        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input = input.trim();
        let args = input.split_whitespace().collect::<Vec<&str>>();

        let mut full_args = vec!["todo-cli"]; // Assuming the binary name is `todo-cli`
        full_args.extend(args);

        match Commands::try_parse_from(full_args) {
            Ok(cli_command) => match cli_command.command {
                SubCommands::Create(args) => {
                    // println!("sub command create is called : {args:?}");
                    create(&mut todo_lists, &args.name);
                }
                SubCommands::List => {
                    view_todolist(&mut todo_lists);
                }
                SubCommands::Complete(args) => {
                    mark_complete(&mut todo_lists, &args.id);
                }
                SubCommands::Delete(args) => {
                    delete_task(&mut todo_lists, &args.id);
                }
                SubCommands::Exit => {
                    println!("Exiting the application.");
                    return;
                },
            },
            Err(e) => e.print().expect("Error printing clap error"),
        }
    }
}
