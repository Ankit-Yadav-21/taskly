mod task_manager;

use std::env;
use task_manager::{add_task, list_tasks, remove_task};

fn print_help() {
    println!(
        "\nTaskly - A CLI Task Manager 🦀🔧\n\
        Usage:\n\
        \tadd <TASK> - Adds a new task\n\
        \tremove <INDEX> - Removes a task by its index\n\
        \tlist - Lists all tasks\n\
        \thelp - Prints this help menu\n"
    );
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        print_help();
        return;
    }

    match args[1].as_str() {
        "add" => {
            if args.len() < 3 {
                eprintln!("Please specify a task to add.");
            } else {
                add_task(args[2..].join(" ")); // Join remaining arguments as the task description
                println!("Task added successfully!");
            }
        }
        "remove" => {
            if args.len() < 3 {
                eprintln!("Please specify the index of the task to remove.");
            } else if let Ok(index) = args[2].parse::<usize>() {
                remove_task(index);
                println!("Task removed successfully!");
            } else {
                eprintln!("Invalid index. Please specify a number.");
            }
        }
        "list" => list_tasks(),
        "help" => print_help(),
        _ => {
            eprintln!("Unknown command. Use 'help' to see the list of commands.");
            print_help();
        }
    }
}
