//WHAT IS task.rs - > anything related to the task program so it is basically what task do

// is it visible?????????I hope so 

//Imp enum and struct 
//enum -> tell all possible value for a field
//struct -> tell the feature of the task

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Task{
    pub id: u32,
    pub title: String,
    pub description: String,
    pub status: Status, //because I will give option in this 
    pub priority: Priority, //same
    pub due_date: Option<String>, //will give option to add due date
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Status{
    Pending,
    Complete
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)] //helps in debugging, storage and equating
pub enum Priority{//Its sequence doesn't matter
    Low,
    Medium,
    High
}