// When we want to delete a task
use std::io::{self,Write};

use crate::storage::{load_tasks,save_tasks};

pub fn delete_task(){
    let mut tasks = load_tasks();

    println!("Delete Task");

    println!("Enter the task ID to delete");
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

    let position = match tasks.iter().position(|task| task.id == id) {
        Some(position) => position,
        None => {
            println!("Task not found");
            return;
        }
    };

    tasks.remove(position);

    save_tasks(&tasks);

    println!("Task deleted");

}