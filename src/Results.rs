


// key Points::
// Result is a enum and i have value of Ok and Err
// Ok---->  The main return value if the code is sucessfull
// Err---> The Err if the code fails


// Impoerint the file system package
use std::fs::read_to_string;

fn main() {
    let filedata: Result<String, std::io::Error> = read_to_string("a.txt");

    match filedata {
        Ok(contents) => {
            println!("File contents:\n{}", contents);
        }
        Err(error) => {
            println!("Failed to read file: {}", error);
        }
    }
}

