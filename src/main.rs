// to-do cli tool 
use chrono::{DateTime, Local};


static mut ALL_TASKS:Vec<Task> = Vec::new();

#[derive(Debug)]
struct Task {
    heading: String, 
    description: String, 
    due: DateTime<Local>,
    completed: bool
}

pub unsafe fn create_task(heading: String, description: String, due: String){
    let due_date = due.parse().unwrap();
    let task: Task  = Task {
        heading: heading, 
        description: description, 
        due:due_date,
        completed: false
    };
    ALL_TASKS.push(task);
}

pub unsafe fn delete_task(id: usize){
    ALL_TASKS.remove(id);
}

pub unsafe fn print_all_tasks(){
    for task in ALL_TASKS.iter(){
        println!("{:?}", task);
    }
}

unsafe fn update_task(id : usize, task: Task){
    ALL_TASKS[id] = task;
}


fn main() {
    
}
