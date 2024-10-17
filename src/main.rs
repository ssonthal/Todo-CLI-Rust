// to-do cli tool 
use std::io;

static mut ALL_TASKS:Vec<Task> = Vec::new();


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

pub unsafe fn delete_task(id: u32) {
    ALL_TASKS.remove((id - 1).try_into().unwrap());
}

pub unsafe fn print_all_tasks(){
    if ALL_TASKS.len() == 0 {
        println!("No tasks found! Please use command to add a new task.\n");
    }
    for task in ALL_TASKS.iter(){
        println!("TaskId: {} \t Description: {} \t Completed: {}", task.id, task.description, task.completed);
    }
}

unsafe fn update_task(id : u32, description: &str){
    let mut iterator = ALL_TASKS.iter_mut(); 
    while let Some(task) = iterator.next() {
        if task.id == id {
            task.description = description.to_string();
            break;
        }
    }
}

fn print_introduction(){
    println!("1 -> add a new task");
    println!("2 -> modify an existing task");
    println!("3 -> mark as task as completed");
    println!("4 -> remove a task");
    println!("5 -> Print all tasks");
    println!("6 -> Exit");
}

unsafe fn take_task_id_input() -> Result<u32, i32> {
    // taking task ID as input
    println!("Please enter ID of the task: \n");
    let mut id_str = String::new();
    io::stdin().read_line(&mut id_str).expect("Error reading line");
    let id:u32 = id_str.trim().parse().expect("ID should be a number.");
    if id > ALL_TASKS.len().try_into().unwrap()  {
        println!("Plase enter a valid ID");
        return Err(-1);
    }
    return Ok(id);
}

unsafe fn take_description_input() -> Result<String, i32> {
    println!("Please enter a description for the task: \n");
    let mut description = String::new();
    io::stdin().read_line(&mut description).expect("Error reading input");
    let description:String = description.trim().to_string();
    return Ok(description);
}

unsafe fn complete_task(id: u32) -> Result<u32, i32> {
    let mut iterator = ALL_TASKS.iter_mut();
    while let Some(task) = iterator.next(){
        if task.id == id {
            task.completed = true;
            break;
        }
    }
    return Ok(0);
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
                let description = take_description_input().unwrap();
                create_task(&description);
            },
            2 => unsafe {
                
                // taking task ID as input
                let id = take_task_id_input().unwrap();

                //taking new description as input
                let description = take_description_input().unwrap();

                //updating the task
                update_task(id, &description);
            },
            3 => unsafe {
                // taking task ID as input
                let id = take_task_id_input().unwrap();
                complete_task(id).unwrap();
                println!("Task marked as completed");

            }, 
            4 => unsafe {
                let id = take_task_id_input().unwrap();
                delete_task(id);
            }, 
            5 => unsafe {
                print_all_tasks();
            }
            6 => 
            {
                println!("Exiting...");
                break;
            }
            _ => 
            {
                println!("Please enter a valid command.");
                continue;
            }
        }
    }
}
