use crate::command::EditArgs;
pub mod command;
use colored::Colorize;
use clap::ValueEnum;


pub mod logger;
use logger::{log_error, log_info, log_notice};

#[derive(Debug)]
pub struct Task {
    id: u32,
    pub name: String,
    pub status: Status,
}

impl Task {
    pub fn new(id: u32, name: String) -> Self {
        Task {
            id,
            name,
            status: Status::PENDING,
        }
    }
}

#[derive(Debug, Clone, ValueEnum)]
pub enum Status{
    PENDING,
    WIP,
    DONE,
    Unknown,
} 

pub fn get_task_by_id<'a>(id: &usize, todo_lists: &'a mut Vec<Task>) -> Option<&'a mut Task> {
    let result = todo_lists.iter_mut().find(|x| x.id == *id as u32);
    match result {
        Some(task) => return Some(task),
        None => None,
    }
}

pub fn create(todo_lists: &mut Vec<Task>, name: &str) {
    if name.is_empty() {
        log_error(format_args!("Task name cannot be empty."));
        return;
    }
    let id = todo_lists.iter().map(|task| task.id).max().unwrap_or(0) + 1;
    let new_task = Task::new(id, name.to_string());
    todo_lists.push(new_task);
    log_info(format_args!("Todo list '{}' created successfully!", name));
}

pub fn edit_task(todo_lists: &mut Vec<Task>, args: EditArgs) {
    let task = get_task_by_id(&args.id, todo_lists);

    match task {
        Some(task) => {
            if let Some(new_name) = args.name {
                task.name = new_name
            }
            if let Some(status) = args.status {
                task.status = status
            }
            log_info(format_args!(
                "Task with ID {} edited successfully!",
                args.id
            ));
        }
        None => log_error(format_args!("No task found with ID {}", args.id)),
    }
}

pub fn mark_complete(todo_lists: &mut Vec<Task>, id: &usize) {
    let task = get_task_by_id(id, todo_lists);

    match task {
        Some(task) => {
            task.status = Status::DONE
        }
        None => log_error(format_args!("No task found")),
    }
    log_info(format_args!("Task marked as completed successfully!"));
    view_todolist(todo_lists);
}

pub fn delete_task(todo_lists: &mut Vec<Task>, id: &usize) {
    if todo_lists.is_empty() {
        log_error(format_args!("No tasks available to delete."));
        return;
    }
    todo_lists.retain(|task| task.id != *id as u32);
    log_info(format_args!("Task deleted successfully!"));
}

pub fn view_todolist(todo_lists: &mut Vec<Task>) {
    if todo_lists.is_empty() {
        log_notice(format_args!("📭 No tasks found. Start by creating one!"));
    } else {
        log_notice(format_args!("📝 Your Todo List:"));

        println!("{:<3} | {:<20} | Status", "ID", "Name");
        for task in todo_lists.iter() {
            let status_display = match task.status {
                Status::PENDING => "⏳ Pending".yellow(),
                Status::WIP => "🔧 In Progress".blue(),
                Status::DONE => "✅ Done".green(),
                Status::Unknown => "❓ Unknown".red(),
            };
            println!(
                "{:<3} | {:<20} | {}",
                task.id,
                task.name,
                status_display
            );
        }
    }
}
