use std::io::{self,Write};

use crate::storage::{load_tasks, save_tasks};
use crate::task::Priority;

pub fn edit_task(){
    let mut tasks = load_tasks();

    println!("Edit tasks");
    println!("Enter the task id"); //According to id someone can edit the task
    io::stdout().flush().unwrap();

    let mut input = String::new();//new means it stored in heap just boasting my knowledge hehe

    io::stdin().read_line(&mut input).expect("Enter the correct id");

    let id: u32 = match input.trim().parse(){
        Ok(id) => id,
        Err(_) => {
            println!("Invalid Id");
            return;
        }
    };

    let task_index = match tasks.iter().position(|task| task.id == id){
        Some(index) => index,
        None => {
            println!("Task not found");
            return;
        }
    };

    loop{
        println!("Edit task");
        println!("Task: {}", tasks[task_index].title);
        println!("1. Edit Title");
        println!("2. Edit Description");
        println!("3. Edit Priority");
        println!("4. Edit Due Date");
        println!("5. Back");

        println!("Choose an option: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("Wrong input");
        let choice: u32 = match input.trim().parse(){
            Ok(choice) => choice,
            Err(_) => {
                println!("Please enter the number");
                continue;
            }
        };

        match choice {
            1 => {
                println!("Current title: {}", tasks[task_index].title);
                println!("Enter the new title");
                io::stdout().flush().unwrap();

                let mut new_title = String::new();
                io::stdin().read_line(&mut new_title).expect("Wrong input");

                let new_title = new_title.trim();

                if new_title.is_empty(){
                    println!("Title cannot be empty");
                    continue;
                }

                tasks[task_index].title = new_title.to_string();

                save_tasks(&tasks);
                println!("Task title updated");

            }

            2 => {
                println!("Current description: {}", tasks[task_index].description);
                println!("Enter the new description");
                io::stdout().flush().unwrap();

                let mut new_description = String::new();
                io::stdin().read_line(&mut new_description).expect("Wrong input");

                let new_description = new_description.trim();

                if new_description.is_empty(){
                    println!("Title cannot be empty");
                    continue;
                }

                tasks[task_index].title = new_description.to_string();

                save_tasks(&tasks);
                println!("Task description updated");

            }

            3 => {
                println!("Current priority: {:?}", tasks[task_index].priority);

                println!("Choose new priority");
                    println!("1. High");
                    println!("2. Medium");
                    println!("3. High");
                    println!("Choose: ");
                    io::stdout().flush().unwrap();

                    let mut new_priority = String::new();
                    io::stdin().read_line(&mut new_priority).expect("Wrong input");

                    let priority: u32 = match new_priority.trim().parse(){
                        Ok(choice) => choice,
                        Err(_) =>{
                            println!("Invalid Priority");
                            continue;
                    }
                };

                let priority = match priority{
                    1 => Priority::High,
                    2 => Priority::Medium,
                    3 => Priority::Low,
                    _ => {
                        println!("Invalid priority");
                        continue;
                    }
                };

                tasks[task_index].priority = priority;

                save_tasks(&tasks);

                println!("Task Priority Updated");
            }

            4 => {
                println!("Current due date: {:?}", tasks[task_index].due_date);

                println!("Enter new due date or leave empty to remove due date");
                io::stdout().flush().unwrap();

                let mut new_due_date = String::new();
                io::stdin().read_line(&mut new_due_date).expect("Wrong input");

                let new_due_date = new_due_date.trim();

                tasks[task_index].due_date = if new_due_date.is_empty(){
                    None}else{
                        Some(new_due_date.to_string())
                    };

                    save_tasks(&tasks);
                    println!("Task due date is updated");
                }

            5 => {
                return;
            }

            _ => {
                println!("Invalid choice");
            }
        }
    

// I was thinking if instead of asking everything to change we give user option to edit whatever the user want to edit
// yUP I am changing the design a little bit

    }

}

// The problem was I was storing task itself in task_index but I wanted to store the position of it