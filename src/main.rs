// Main starting 
use std::io::{self, Write};

mod task;//tell rust module task.rs exists
mod commands;
mod storage;

fn main() {
    loop{
        println!("\nWELCOME TO MY CLI TODO APP");
        println!("1. ADD TASK");
        println!("2. EDIT TASK");
        println!("3. VIEW TASKS");
        println!("4. MARK AS COMPLETE/ INCOMPLETE");
        println!("5. DELETE TASK");
        println!("6. EXIT");

        println!("Choose an Option: ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        //now we have to take input
        io::stdin().read_line(&mut input).expect("Wrong answer");

        let choice: u32 = match input.trim().parse(){
            Ok(choice) => choice,
            Err(_) => {
                println!("Please enter a number.");
                continue;
            }
        };

        //now we have to connect number to the commands

        //we use match for that

        match choice{
            1 => commands::add::add_task(),
            2 => commands::edit::edit_task(),
            3 => commands::view::view_task(),
            4 => commands::status::change_status(),
            5 => commands::delete::delete_task(),
            6 => {
                println!("Byeee!!!");
                break;
            }
            _ => println!("Invalid command"),
        }
    }

}


