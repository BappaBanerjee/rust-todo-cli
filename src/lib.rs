pub mod command;

#[derive(Debug)]
pub struct Task {
    id: u32,
    pub name: String,
    pub completed: bool,
}

pub fn create(todo_lists: &mut Vec<Task>, name: &str) {
    if name.is_empty() {
        println!("Task name cannot be empty.");
        return;
    }
    let id = todo_lists.iter().map(|task| task.id).max().unwrap_or(0) + 1;
    let new_task = Task {
        id,
        name: name.to_string(),
        completed: false,
    };
    todo_lists.push(new_task);
    println!("Todo list '{}' created successfully!", name);
}

pub fn view_todolist(todo_lists: &mut Vec<Task>) {
    println!("*************My Todos***************");
    println!("====================================");

    if todo_lists.is_empty() {
        println!("No todo lists available.");
    } else {
        println!("Index |   ID  |   Name    |   Status");
        println!("====================================");
        for (index, task) in todo_lists.iter().enumerate() {
            println!(
                "{}    |   {}  |   {}  |   {}",
                index + 1,
                task.id,
                task.name,
                if task.completed {
                    "Completed"
                } else {
                    "Not Completed"
                }
            );
        }
    }
    println!("====================================");
}

pub fn mark_complete(todo_lists: &mut Vec<Task>, index: &usize) {
    let task = todo_lists.get_mut(index - 1);
    match task {
        Some(task) => {
            task.completed = true;
        }
        None => println!("No task found"),
    }
    println!("Task marked as completed successfully!");
    view_todolist(todo_lists);
}

pub fn delete_task(todo_lists: &mut Vec<Task>, index: &usize) {
    if index > &todo_lists.len() {
        println!("Invalid task index.");
        return;
    }
    if todo_lists.is_empty() {
        println!(
            "No tasks available to delete."
        );
        return;
    }
    todo_lists.remove(index - 1);
    println!("Task deleted successfully!");
    view_todolist(todo_lists);
}

pub fn help() {
    println!("Available commands:");
    println!("1. create <name> - Create a new todo list with the given name.");
    println!("2. list - List all todo lists.");
    println!("3. complete <id> - Mark the todo list with the given ID as completed.");
    println!("4. delete <id> - Delete the todo list with the given ID.");
    println!("5. exit - Exit the application.");
}