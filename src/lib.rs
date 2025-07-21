use colored::*;

use crate::command::{EditArgs};

use std::fmt::{Arguments};

pub mod command;

#[derive(Debug)]
pub struct Task {
    id: u32,
    pub name: String,
    pub completed: bool,
}

impl Task {
    pub fn new(id: u32, name: String) -> Self {
        Task {
            id,
            name,
            completed: false,
        }
    }
}

pub fn get_task_by_id<'a>(id : &usize, todo_lists: &'a mut Vec<Task>) -> Option<&'a mut Task>{
    let result  = todo_lists.iter_mut().find(|x| x.id == *id as u32);
    match result {
        Some(task) => return Some(task),
        None => None
    }
}

pub fn log_error(args : Arguments) {
    // eprintln!("{}", format!("Error: {}", message).red().bold());
    eprintln!("{}", format!("Error: {}", args).red().bold());
}

pub fn create(todo_lists: &mut Vec<Task>, name: &str) {
    if name.is_empty() {
        log_error(format_args!("Task name cannot be empty."));
        return;
    }
    let id = todo_lists.iter().map(|task| task.id).max().unwrap_or(0) + 1;
    let new_task = Task::new(id, name.to_string());
    todo_lists.push(new_task);
    println!("Todo list '{}' created successfully!", name);
}

pub fn edit_task(todo_lists: &mut Vec<Task>, args : EditArgs){
    // let task = todo_lists.get_mut(args.id);

    let task = get_task_by_id(&args.id, todo_lists);

    match task {
        Some(task) => {
            if let Some(new_name) = args.name {
                task.name = new_name
            }
            if let Some(completed) = args.completed {
                task.completed = completed
            }
        },
        None => log_error(format_args!("No task found with ID {}", args.id)),
    }
}

pub fn mark_complete(todo_lists: &mut Vec<Task>, index: &usize) {
    let task = todo_lists.get_mut(index - 1);
    match task {
        Some(task) => {
            task.completed = true;
        }
        None => log_error(format_args!("No task found")),
    }
    println!("Task marked as completed successfully!");
    view_todolist(todo_lists);
}

pub fn delete_task(todo_lists: &mut Vec<Task>, index: &usize) {
    if index > &todo_lists.len() {
        log_error(format_args!("Invalid task index."));
        return;
    }
    if todo_lists.is_empty() {
        log_error(format_args!( "No tasks available to delete."));
        return;
    }
    todo_lists.remove(index - 1);
    println!("Task deleted successfully!");
    view_todolist(todo_lists);
}


pub fn view_todolist(todo_lists: &mut Vec<Task>) {
    println!("*************My Todos***************");
    println!("====================================");

    if todo_lists.is_empty() {
        log_error(format_args!("No todo lists available."));
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


pub fn help() {
    println!("Available commands:");
    println!("1. create <name> - Create a new todo list with the given name.");
    println!("2. list - List all todo lists.");
    println!("3. complete <id> - Mark the todo list with the given ID as completed.");
    println!("4. delete <id> - Delete the todo list with the given ID.");
    println!("5. exit - Exit the application.");
}