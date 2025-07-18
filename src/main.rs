//todo cli app
#[derive(Debug)]
struct Task {
    // id : u32,
    name: String,
    completed: bool,
}
fn main() {
    let mut todo_lists: Vec<Task> = Vec::new();

    loop {
        println!("Welcome to the Todo CLI App!");
        println!("1. Create a new todo list");
        println!("2. View todo lists");
        println!("3. Marking as Complete");
        println!("4. Delete a task");
        println!("5. Exit");

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

                // Here you would typically gather input for the new todo list
                let mut todo_name = String::new();
                println!("Enter the name of the task");
                std::io::stdin()
                    .read_line(&mut todo_name)
                    .expect("Failed to read line");
                let todo_name = todo_name.trim();
                if todo_name.is_empty() {
                    println!("Task name cannot be empty.");
                    continue;
                }
                let new_task = Task {
                    name: todo_name.to_string(),
                    completed: false,
                };
                todo_lists.push(new_task);
                println!("Todo list '{}' created successfully!", todo_name);
            }
            2 => {
                println!("Viewing todo lists...");
                
                println!("**********My Todos*********");
                println!("===========================");

                if todo_lists.is_empty() {
                    println!("No todo lists available.");
                } else {
                    for (index, task) in todo_lists.iter().enumerate() {
                        println!(
                            "{} | {} | {}",
                            index + 1,
                            task.name,
                            if task.completed {
                                "Completed"
                            } else {
                                "Not Completed"
                            }
                        );
                    }
                }
                println!("===========================");
            }
            3 => {
                // marking as complete
                let mut index = String::new();
                println!("Enter the task Id");
                std::io::stdin().read_line(&mut index).expect("cannot read input");
                println!("{index}");
                let index : usize = index.trim().parse().expect("invalid number");
                if todo_lists.len() < index {
                    println!("smallest")
                }

                let task = todo_lists.get_mut(index - 1);
                match task {
                    Some(task ) => {
                        task.completed = true;
                    },
                    None => println!("No task found")
                }
            }
            4 => {
                //deleting an item
                let mut index = String::new();
                std::io::stdin().read_line(&mut index).expect("unable to read");
                let index : usize = index.trim().parse().expect("invalid number");
                todo_lists.remove(index - 1);
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
