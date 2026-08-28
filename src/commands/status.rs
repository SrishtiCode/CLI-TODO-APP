// For the complete and incomplete and removing the complete task
use std::io::{self, Write};

use crate::storage::{load_tasks,save_tasks};
use crate::task::Status;

pub fn change_status(){
    let mut tasks = load_tasks();

    println!("Change Task Status");

    println!("Enter the task ID");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Wrong input");

    let id: u32 = match input.trim().parse(){
        Ok(id) => id,
        Err(_) => {
            println!("Invalid ID");
            return;
        }
    };

    let task = match tasks.iter_mut().find(|task| task.id == id){
        Some(task) => task,
        None => {
            println!("Task not found");
            return;
        }
    };

    match task.status {
        Status::Pending => {
            task.status = Status::Complete;
            println!("Task marked as complete");
        }
        Status::Complete => {
            task.status = Status::Pending;
            println!("Task marked as Pending");
        }
    } 

    save_tasks(&tasks);
}