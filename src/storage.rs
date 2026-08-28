// To save the data 

use crate::task::Task;

pub fn save_tasks(tasks: &Vec<Task>){ // I am using vector to store the tasks
    let json = serde_json::to_string_pretty(tasks).expect("Failed to serialize tasks"); //change from rust to json

    if let Err(error) = std::fs::write("tasks.json",json){
        eprintln!("Failed to save tasks: {}" ,error)//will print the error
    }
}

pub fn load_tasks() -> Vec<Task> { //I did'nt define the output type so it was taking it in u32 instead of vec
    let data = match std::fs::read_to_string("tasks.json"){ // putting json data into rust vectors
        Ok(data) => data,
        Err(_) => return Vec::new(),
    };

    match serde_json::from_str(&data){
        Ok(tasks) => tasks,
        Err(error) => {
            eprintln!("Failed to load tasks: {}", error);
            Vec::new()
        }
    }
}


