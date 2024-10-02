use serde::{Deserialize, Serialize};
use std::fs::{self, OpenOptions};
use std::io::{self, BufReader};

const TASK_FILE: &str = "tasks.json";

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    description: String,
    completed: bool,
}

// Function to read tasks from the file
fn read_tasks() -> Vec<Task> {
    let file = OpenOptions::new().read(true).open(TASK_FILE);
    match file {
        Ok(file) => {
            let reader = BufReader::new(file);
            serde_json::from_reader(reader).unwrap_or_else(|_| Vec::new())
        }
        Err(_) => Vec::new(),
    }
}

// Function to write tasks to the file
fn write_tasks(tasks: &Vec<Task>) -> io::Result<()> {
    let json = serde_json::to_string_pretty(tasks)?;
    fs::write(TASK_FILE, json)?;
    Ok(())
}

// Function to add a new task
pub fn add_task(description: String) {
    let mut tasks = read_tasks();
    let new_task = Task {
        description,
        completed: false,
    };
    tasks.push(new_task);
    if let Err(e) = write_tasks(&tasks) {
        eprintln!("Failed to write task: {}", e);
    }
}

// Function to remove a task by index
pub fn remove_task(index: usize) {
    let mut tasks = read_tasks();
    if index < tasks.len() {
        tasks.remove(index);
        if let Err(e) = write_tasks(&tasks) {
            eprintln!("Failed to remove task: {}", e);
        }
    } else {
        eprintln!("Invalid index.");
    }
}

// Function to list all tasks
pub fn list_tasks() {
    let tasks = read_tasks();
    if tasks.is_empty() {
        println!("No tasks found.");
    } else {
        for (index, task) in tasks.iter().enumerate() {
            println!(
                "{}. [{}] - {}",
                index,
                if task.completed { "x" } else { " " },
                task.description
            );
        }
    }
}

// Function to update a task by index
pub fn update_task(index: usize, new_description: Option<String>, completed: Option<bool>) {
    let mut tasks = read_tasks();
    if index < tasks.len() {
        if let Some(description) = new_description {
            tasks[index].description = description;
        }
        if let Some(is_completed) = completed {
            tasks[index].completed = is_completed;
        }
        if let Err(e) = write_tasks(&tasks) {
            eprintln!("Failed to update task: {}", e);
        } else {
            println!("Task updated successfully!");
        }
    } else {
        eprintln!("Invalid index. No task found at this index.");
    }
}