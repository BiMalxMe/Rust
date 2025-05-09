
use std::io::{self, Write};

use chrono::NaiveDate;

struct Tasks {
    title: String,
    description: String,
    deadline: NaiveDate,
    completed: bool,
}

impl Tasks {

    fn add_new_task(title: String, description: String, deadline: NaiveDate) -> Self{
      Self{
        title,
            description,
            deadline,
            completed: false,
      }
    }
  
    fn mark_completed(&mut self) {
        self.completed = true;
        println!("Updated the Task")
    }
}
fn read_input(prompt : &str ) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}
    fn main(){
        let mut vec = Vec::new();
      
    loop {
        println!("\n--- Task Manager CLI ---");
        println!("1. Add Task");
        println!("2. List Tasks");
        println!("3. Mark Task as Complete");
        println!("4. Exit");

        let choice: String = read_input("Enter your choice ");
        // pattern expects srt for easy
        match choice.as_str() {
            "1" => {
                let title = read_input("Enter the title : ");
                let description = read_input("Enter description: ");
                let deadline_input = read_input("Enter deadline (YYYYMMDD): ");
                let deadline = NaiveDate::parse_from_str(&deadline_input, "%Y%m%d").unwrap();

                let task = Tasks::add_new_task(title, description, deadline);
                vec.push(task);
            }
            "2" => {
                if !vec.is_empty() {
                    for (index, value) in vec.iter().enumerate() {
                        let stringformat = format!(
                            "Tasks #{}:\n--------------\ntitle: {}\ndescription: {}\ndeadline: {}\ncompleted: {}\n",
                            index + 1, 
                            value.title, 
                            value.description, 
                            value.deadline, 
                            value.completed
                        );
                        println!("{}", stringformat); 
                    }
                } else {
                    println!("No tasks available.");
                }
            }
            "3" => {
                let index = read_input("Enter the index todo thats completed : ")
                    .parse::<usize>().unwrap();
                if index > 0 && index <= vec.len() {
                    vec[index - 1].mark_completed();
                    println!("Task marked as complete!");
                } else {
                    println!("Invalid task number.");
                }
            }
            "4" => {
                println!("Exiting...");
                break;
            }
            _ => {
                println!("Invalid")
            }
        }
    }
    
    }

