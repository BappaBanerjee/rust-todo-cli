//todo cli app
use todo_cli::Task;
use std::io::{self, Write};

use todo_cli::{create, delete_task, mark_complete, view_todolist };
fn main() {
    let mut todo_lists: Vec<Task> = Vec::new();

    loop {
         io::stdout().flush().unwrap();

        let mut input  = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        
        let input = input.trim();
        let mut parts = input.split_whitespace();
        let command = parts.next(); 
        let args: Vec<&str> = parts.collect();

        match command {
            Some("create") => {
                println!("Creating a new todo list...");
                create(&mut todo_lists);
            }
            Some("view") => {
                println!("Viewing todo lists...");
                view_todolist(&mut todo_lists);
            }
            Some("complete") => {
                // marking as complete
                mark_complete(&mut todo_lists);
            }
            Some("delete") => {
                //deleting an item
                delete_task(&mut todo_lists);
            }
            Some("exit") => {
                println!("Exiting the app.");
                break;
            }
            _ => {
                println!("Invalid choice, please try again.");
            }
        }
    }
}
