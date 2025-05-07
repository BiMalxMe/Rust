
use std::io;

fn main(){
    let mut input : String = String::new();
    println!("Enter Any one Number 1-7 to Find week days ");
    println!("");
    io::stdin()
     .read_line(&mut input)
     .expect("Failed to read line");
    let finalval:u8 = input.trim().parse().expect("Error While Parsing");

    //Once great Thing:
    //We dont have to explecitely use break after one val is mached
    // Rust automatically cancels the process after the one value is found

    match finalval {
        1 => println!("Sunday"),
        2 => println!("Monday"),
        3 => println!("Tuesday"),
        4 => println!("Wednesday"),
        5 => println!("THursday"),
        6 => println!("Friday"),
        7 => println!("Saturday"),

        //other inputs
        _=> println!("Error")
    }
}