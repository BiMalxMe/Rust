use crossterm::{
    event::{self, Event, KeyCode, KeyEvent},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use std::io;

fn main() -> io::Result<()> {
    println!("Enter any key to start and Esc to end");

    //used to enter the value without entering
    enable_raw_mode()?;

    loop {
        //hits every 500 reaconds - is a npn blocking 
        //But in most real apps, using poll() is the preferred way to stay non-blocking and responsive.
        if event::poll(std::time::Duration::from_millis(500))? {
            if let Event::Key(KeyEvent { code, .. }) = event::read()? {
                match code {
                    KeyCode::Esc => {
                        println!("Exiting..");
                        break;
                    }
                    KeyCode::Char(c) => {
                        println!("You pressed a key {}", c);
                    }
                    _ => {} // Handle other key codes if needed
                }
            }
        }
    }

    disable_raw_mode()?;
    Ok(())
}