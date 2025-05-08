use std::fs::File;
use std::io::{self, Read, Write};

fn main() -> io::Result<()> {
    // Create and write to a file
    let mut file = File::create("hello.txt")?;
    file.write_all(b"Hello, Rust!")?;
    
    // Read from a file
    let mut file = File::open("hello.txt")?;
    let mut contents = String::new();
    //move the readed data into the content var
    file.read_to_string(&mut contents)?;
    println!("File contains: {}", contents);
    
    Ok(())
}