use clap::Parser;
use std::io::{self, Write};
use todo_cli::command::{Commands, SubCommands};
use todo_cli::{
    Task, create, delete_task, edit_task,
    logger::{log_error, log_info},
    mark_complete, view_todolist,
};

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

        let mut full_args = vec!["todo-cli"]; // adding the binary name `todo-cli`
        full_args.extend(args);

        match Commands::try_parse_from(full_args) {
            Ok(cli_command) => match cli_command.command {
                SubCommands::Create(args) => {
                    create(&mut todo_lists, &args.name);
                }
                SubCommands::List => {
                    view_todolist(&mut todo_lists);
                }
                SubCommands::Edit(args) => {
                    edit_task(&mut todo_lists, args);
                }
                SubCommands::Complete(args) => {
                    mark_complete(&mut todo_lists, &args.id);
                }
                SubCommands::Delete(args) => {
                    delete_task(&mut todo_lists, &args.id);
                }
                SubCommands::Exit => {
                    log_info(format_args!("Exiting the application."));
                    return;
                }
            },
            Err(e) => log_error(format_args!("Error parsing command: {}", e)),
        }
    }
}
