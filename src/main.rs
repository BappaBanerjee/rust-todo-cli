//todo cli app
use todo_cli::Task;

use todo_cli::{create, delete_task, mark_complete, print_commands, view_todolist};
fn main() {
    let mut todo_lists: Vec<Task> = Vec::new();

    loop {
        print_commands();

        let mut choice = String::new();
        std::io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");
        
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input, please enter a number.");
                continue;
            }
        };

        match choice {
            1 => {
                println!("Creating a new todo list...");
                create(&mut todo_lists);
            }
            2 => {
                println!("Viewing todo lists...");
                view_todolist(&mut todo_lists);
            }
            3 => {
                // marking as complete
                mark_complete(&mut todo_lists);
            }
            4 => {
                //deleting an item
                delete_task(&mut todo_lists);
            }
            5 => {
                println!("Exiting the app.");
                break;
            }
            _ => {
                println!("Invalid choice, please try again.");
            }
        }
    }
}
