// to-do cli tool 
use std::io::{self, Write};

use clap::error;


static mut ALL_TASKS:Vec<Task> = Vec::new();

#[derive(Debug)]
struct Task {
    id: u32,
    description: String, 
    completed: bool
}

pub unsafe fn create_task(description: &str) {
    let id = ALL_TASKS.len() as u32 + 1;
    let task: Task  = Task {
        id: id,
        description: description.to_string(), 
        completed: false
    };
    ALL_TASKS.push(task);
}

pub unsafe fn delete_task(id: u32){
    ALL_TASKS.remove(id.try_into().unwrap());
}

pub unsafe fn print_all_tasks(){
    for task in ALL_TASKS.iter(){
        println!("Task: Id {} Description {} Completed: {}", task.id, task.description, task.completed);
    }
}

unsafe fn update_task(id : usize, task: Task){
    ALL_TASKS[id] = task;
}

fn print_introduction(){
    println!("1 -> add a new task");
    println!("2 -> modify an existing task");
    println!("3 -> mark as task as completed");
    println!("4 -> remove a task");
    println!("5 -> Print all tasks");
    println!("6 -> Exit");
}

unsafe fn take_task_id_input() -> Result<u32, err> {
    // taking task ID as input
    println!("Please enter ID of the task: \n");
    let mut id_str = String::new();
    io::stdin().read_line(&mut id_str).expect("Error reading line");
    let id:u32 = id_str.trim().parse().expect("ID should be a number.");
    if (id > ALL_TASKS.len()){
        println!("Plase enter a valid ID");
        return error;
    }
    return Result(id);
}


fn main() {
    println!("Welcome to your To-do CLI application!!!");
    loop{
        print_introduction();
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Error reading input");
        let choice:u32 = choice.trim().parse().expect("Entry should be a number");

        match choice {
            1 => unsafe {
                println!("Please enter a description for the task: \n");
                let mut description = String::new();
                io::stdin().read_line(&mut description).expect("Error reading input");
                let description:String = description.trim().to_string();
                create_task(&description);
            },
            2 => unsafe {
                
                // taking task ID as input
                println!("Please enter ID of the task: \n");
                let mut id_str = String::new();
                io::stdin().read_line(&mut id_str).expect("Error reading line");
                let id:u32 = id_str.trim().parse().expect("ID should be a number.");
                if (id > ALL_TASKS.len()){
                    println!("Plase enter a valid ID");
                    continue;
                }

                //taking new description as input
                println!("Please enter the new description: \n");
                let mut new_description = String::new();
                io::stdin().read_line(&mut new_description).expect("Error reading line");
                new_description = new_description.trim().to_string();

                //updating the task
                update_task(id, task);
            },

            3 => unsafe {
                // taking task ID as input
                println!("Please enter ID of the task: \n");
                let mut id_str = String::new();
                io::stdin().read_line(&mut id_str).expect("Error reading line");
                let id:u32 = id_str.trim().parse().expect("ID should be a number.");
                if (id > ALL_TASKS.len()){
                    println!("Plase enter a valid ID");
                    continue;
                }
                ALL_TASKS[id].completed = true;
                println!("Task marked as completed");

            }, 
            4 => {

            }
            6 => 
            {
                println!("Exiting...");
                break;
            }
            _ => {println!("nothing")}
        }
    }
}
