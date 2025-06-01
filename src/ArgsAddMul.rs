//Gives acess to sommand line interfaces
use std::env;

fn main() {
    //env::args(): Returns an iterator of command-line arguments passed to the program.
    // collect collects in the vectr
    let args: Vec<String> = env::args().collect();

    // if not four args
    if args.len() != 4 {
        eprintln!("Usage: {} <add|mul> <num1> <num2>", args[0]);
        //exit the process
        //Immediately stops the program and returns control to the operating system.
        // 1 means Error
        // 0 means no error OR "success"
        std::process::exit(1);
    }

    //let second args
    let op = &args[1];
    //parse into the integer
    let num1: i32 = args[2].parse().expect("Invalid number");
    let num2: i32 = args[3].parse().expect("Invalid number");

    //solve accordingly
    match op.as_str() {
        "add" => println!("Result: {}", num1 + num2),
        "mul" => println!("Result: {}", num1 * num2),
        // if other is given exept the add and mul then
        _ => eprintln!("Unknown operation. Use 'add' or 'mul'."),
    }
}

//--------      PROCESS OF EXCECUTION       ------------//
// FIRSTLY BUILD THE RS FILE
//      rustc ArgsAddMul.rs

//THEN
//      ./ArgsAddMul mul  6 7
//          OR
//          ./ArgsAddMul add  6 7