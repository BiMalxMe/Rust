
//Simple code to print something in the terminal using the crossterm 

use std::io::stdout;

// Import `execute` macro and `Print` command from the Crossterm library
use crossterm::{execute, style::Print};

fn main() {
    // For gives output in console or the terminal
    let mut stdout = stdout();

   //execute sent the terminal differnt commands
   // it is giving command to print the helo from ..... in the terminal
    execute!(stdout, Print("Hello from Crossterm!\n")).unwrap();
}
