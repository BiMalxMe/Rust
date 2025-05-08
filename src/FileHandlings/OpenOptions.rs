// In short, OpenOptions provides flexibility and control 
// for more nuanced file operations, and it’s a safer,
// cleaner way to handle advanced use cases in Rust.

use std::{fs::OpenOptions, io::Write};
 
fn main() -> std::io::Result<()>{
    let mut file = OpenOptions::new()
    .append(true)//can append file
    .create(true)//create file if not exist
    .open("xyx.txt")?;//open file
    
    let _ = file.write_all("Bimalchalise".as_bytes());
    Ok(())
}