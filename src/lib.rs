#[derive(Debug)]
pub struct Task {
    // id : u32,
    pub name: String,
    pub completed: bool,
}

pub fn print_commands() {
    println!("Welcome to the Todo CLI App!");
    println!("1. Create a new todo list");
    println!("2. View todo lists");
    println!("3. Marking as Complete");
    println!("4. Delete a task");
    println!("5. Exit");
}

pub fn create(todo_lists: &mut Vec<Task>) {
    let mut todo_name = String::new();
    println!("Enter the name of the task");
    std::io::stdin()
        .read_line(&mut todo_name)
        .expect("Failed to read line");
    let todo_name = todo_name.trim();
    if todo_name.is_empty() {
        println!("Task name cannot be empty.");
        return;
    }
    let new_task = Task {
        name: todo_name.to_string(),
        completed: false,
    };
    todo_lists.push(new_task);
    println!("Todo list '{}' created successfully!", todo_name);
}

pub fn view_todolist(todo_lists: &mut Vec<Task>) {
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

pub fn mark_complete(todo_lists: &mut Vec<Task>) {
    let mut index = String::new();
    println!("Enter the task Id");
    std::io::stdin()
        .read_line(&mut index)
        .expect("cannot read input");
    println!("{index}");
    let index: usize = index.trim().parse().expect("invalid number");
    if todo_lists.len() < index {
        println!("smallest")
    }

    let task = todo_lists.get_mut(index - 1);
    match task {
        Some(task) => {
            task.completed = true;
        }
        None => println!("No task found"),
    }
}

pub fn delete_task(todo_lists: &mut Vec<Task>) {
    let mut index = String::new();
    std::io::stdin()
        .read_line(&mut index)
        .expect("unable to read");
    let index: usize = index.trim().parse().expect("invalid number");
    todo_lists.remove(index - 1);
}
