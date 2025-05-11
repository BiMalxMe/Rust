use std::io::stdout;               
use crossterm::{execute, cursor::MoveTo};  

fn main() {
    //create multiple output handles as it can be acessed many times
    let mut out = stdout(); 

    // Move the cursor to column 30, row 5 in terminal
    execute!(out, MoveTo(0, 99)).unwrap();

    // Now print the string "Hello, world!" at the current cursor position
    execute!(out, crossterm::style::Print("Hello, world!")).unwrap();
}
