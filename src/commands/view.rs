use std::io::{self, Write};

use crate::storage::load_tasks;
use crate::task::{Task, Priority, Status};

fn view_task_list(tasks: &[Task]){
    for task in tasks{
        println!("-----------");
        let status = match task.status{
            Status::Pending => "[]",
            Status::Complete => "[x]"
        };
        println!("{} {} - {}", status, task.id, task.title);
        println!("Description: {}", task.description);
        println!("Priority: {:?}", task.priority);
        println!("Due date: {:?}", task.due_date);
        println!("Created at: {}", task.created_at);
    }
}

pub fn view_task(){
    loop{
        println!("\n View Tasks");
        println!("1. View all");
        println!("2. Filter by status");
        println!("3. Filter by priority");
        println!("4. Search");
        println!("5. Sort");
        println!("6. Back");

        println!("Choose:");
        io::stdout().flush().unwrap();

        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("Wrong");
        let choice: u32 = match input.trim().parse(){
            Ok(choice) => choice,
            Err(_) => {
                println!("Please enter a number.");
                continue;
            }
        };

        match choice{
            1 => view_all(),
            2 => filter_by_status(),
            3 => filter_by_priority(),
            4 => search_tasks(),
            5 => sort_tasks(),
            6 => break,
            _ => println!("Invalid choice"),
        }

    }
}

pub fn view_all(){
    let tasks = load_tasks();

    if tasks.is_empty(){
        println!("No task found.");
        return;
    }
    view_task_list(&tasks);
}

pub fn filter_by_status(){
    let tasks = load_tasks();

    println!("Filter by Status");
    println!("1. Pending");
    println!("2. Complete");
    println!("Choose: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Wrong input");
     
    let choice: u32 = match input.trim().parse(){
        Ok(choice) => choice,
        Err(_) => {
            println!("Please enter a number.");
            return;
        }
    };

    let status = match choice{
        1 => Status::Pending,
        2 => Status::Complete,
        _ => {
            println!("Invalid choice");
            return;
        }
    }; 

    let filtered_tasks: Vec<Task> = tasks.into_iter().filter(|task| task.status == status).collect();
    view_task_list(&filtered_tasks);

}

pub fn filter_by_priority(){
    let tasks = load_tasks();

    println!("Filter by Priority");
    println!("1. High");
    println!("2. Medium");
    println!("3. Low");
    println!("Choose: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(& mut input).expect("Wrong input");

    let choice: u32 = match input.trim().parse(){
        Ok(choice) => choice,
        Err(_) => {
            println!("Please enter a number.");
            return;
        }
    };

    let priority = match choice {
        1 => Priority::High,
        2 => Priority::Medium,
        3 => Priority::Low,
        _ => {
            println!("Invalid choice");
            return;
        }
        
    };

    let filtered_tasks: Vec<Task> = tasks.into_iter().filter(|task| task.priority == priority).collect();
    view_task_list(&filtered_tasks);

}

pub fn search_tasks(){
    //so we take any keyword - it should be case insensitive and then find according to it

    let tasks = load_tasks();

    println!("Enter the keyword to seach");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(& mut input).expect("Wrong input");

    let keyword = input.trim().to_lowercase();

    let filtered_tasks: Vec<Task> = tasks.into_iter().filter(|task| {task.title.to_lowercase().contains(&keyword) || task.description.to_lowercase().contains(&keyword)}).collect();

    if filtered_tasks.is_empty(){
        println!("Not present");
        return;
    } 

    view_task_list(&filtered_tasks);
}

pub fn sort_tasks(){
    let mut tasks = load_tasks();

    println!("Sort tasks");
    println!("1. Priority");
    println!("2. Due Date");//due date is optional so when we compare it can be none so it can be like (Some , None), (None,None), (None, Some) and (Some, Some) so we have to make four options
    println!("3. Created Date");
    println!("4. Back");
    println!("Choose: ");

    io::stdout().flush().unwrap();

    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Wrong input");

    let choice: u32 = match input.trim().parse(){
        Ok(choice) => choice,
        Err(_) => {
            println!("Please enter a number.");
            return;
        }
    };

    match choice {
        1 => {
            tasks.sort_by(|a,b| b.priority.cmp(&a.priority));//descending order
            view_task_list(&tasks);
        }

        2 => {
            tasks.sort_by(|a,b|{
                match (&a.due_date, &b.due_date){
                    (None,None) => std::cmp::Ordering::Equal,
                    (None, Some(_)) => std::cmp::Ordering::Less,
                    (Some(_), None) => std::cmp::Ordering::Greater,
                    (Some(a_date),Some(b_date)) => a_date.cmp(b_date),//ascending order
                }
            });
            view_task_list(&tasks);
        }

        3 => {
            tasks.sort_by(|a,b| b.created_at.cmp(&a.created_at));//ascending order
            view_task_list(&tasks);
        }

        4 => return,

        _ => println!("Invalid choice"),
    }
}

