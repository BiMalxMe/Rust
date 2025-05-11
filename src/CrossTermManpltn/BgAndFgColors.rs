use std::io::stdout;

use crossterm::{
    execute,
    style::{Color, Print, SetBackgroundColor, SetForegroundColor},
};

fn main() {
    //making mutbale handle to use multiple time with differnt output format
    let mut out = stdout();

    //Change bg and fg color
    execute!(
        out,
        SetBackgroundColor(Color::DarkGrey),
        SetForegroundColor(Color::DarkCyan)
    )
    .unwrap();
    // Print to check the code 
    execute!(out, Print("I am a modified one\n")).unwrap();
    //Reset The code into its default state
    execute!(
        out,
        SetBackgroundColor(Color::Reset),
        SetForegroundColor(Color::Reset)
    )
    .unwrap();
    //test the changes after the reset
    execute!(out, Print("I am here after getting reset")).unwrap();
}
