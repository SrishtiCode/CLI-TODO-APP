use std::io::{self, Write};
use chrono::{Local, NaiveDate};
use crate::task::{Task, Status, Priority};
use crate::storage::{save_tasks, load_tasks};

pub fn add_task(){

    let mut tasks = load_tasks();
    
    let next_id= tasks.iter().map(|task| task.id).max().unwrap_or(0) + 1; // if the id is implementing first time and then it will automatically add it
    
    println!("Enter the title: ");

    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Wrong input");

    let title = input.trim().to_string();

    if title.is_empty(){
        println!("Title cannot be empty");
        return;
    }

    println!("Enter the description: ");

    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Wrong input");

    let description = input.trim().to_string();

    if description.is_empty(){
        println!("Description cannot be empty");
        return;
    }

    println!("Enter the priority:\n\
    1. High,\n\
    2. Medium,\n\
    3. Low\n\
    Choose: ");

    io::stdout().flush().unwrap(); //It helps like if there is a output in the buffer and not printing it flushes it

    let mut priority = String::new();

    io::stdin().read_line(&mut priority).expect("Wrong input");

    let priority: u32 = priority.trim().parse().expect("Please enter a number");

    let priority = match priority {
        1 => Priority::High,
        2 => Priority::Medium,
        3 => Priority::Low,
        _ => panic!("Invalid choice"), //we said panic here instead of println because println will return () -> unit value instead of value
    };

    println!("Enter due date (optional):");
    io::stdout().flush().unwrap();

    let mut due_date = String::new();

    io::stdin().read_line(&mut due_date).expect("Wrong input");

    let due_date = due_date.trim();

    //now it can be or cannot be so we will use some here

    let due_date = if due_date.is_empty(){
        None
    }else{
        match NaiveDate::parse_from_str(due_date, "%Y-%m-%d") {
            Ok(_) => Some(due_date.to_string()),
            Err(_) => {
                println!("Invalid date. Use YYYY-MM-DD");
                return;
            }
        }
    };

    let created_at = Local::now().to_rfc3339();//it will need to have time

    let task = Task{
        id: next_id,
        title: input.trim().to_string(),
        description: description.trim().to_string(),
        status: Status::Pending, //by default
        priority,
        due_date,
        created_at,
    };

    tasks.push(task);
    save_tasks(&tasks);

    println!("Task added successfully!");

}
// I am just staring screen why there are errors