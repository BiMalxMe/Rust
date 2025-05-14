use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Terminal,
    layout::{Constraint, Direction, Layout},
    prelude::CrosstermBackend,
    text::{Line, Text},
    widgets::{Block, Borders, Paragraph},
};

fn main() -> std::io::Result<()> {
    // This vec holds the Total lists
    let mut alllist: Vec<String> = Vec::new();
    // Making the code run on typing .. without Enter
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    //enter the alternate screen
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut input_text = String::new();

    loop {
        terminal.draw(|f| {
            let size = f.size();

            // main chunk to divide
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(70), Constraint::Percentage(20)])
                .margin(1)
                .split(size);

            // Converting the string into str in vector as the Line expects
            let lines = alllist
                .iter()
                .enumerate()
                .map(|(i, s)| Line::from(format!("{}. {}", i + 1, s)))
                //change into the vector
                .collect::<Vec<_>>();
            //first chunk
            let paragraph1 = Paragraph::new(Text::from(lines)).block(
                Block::default()
                    .title(" Total Lists ")
                    .borders(Borders::ALL),
            );
            //second chunck
            let paragraph2 = Paragraph::new(Line::from(input_text.clone())).block(
                Block::default()
                    .title("Add new List ------ Enter 'clear' to Clear ")
                    .borders(Borders::ALL),
            );

            //rendering
            f.render_widget(paragraph1, chunks[0]);
            f.render_widget(paragraph2, chunks[1]);
        })?;
        // giving events such that code runs
        if let Event::Key(key_event) = event::read()? {
            match key_event.code {
                KeyCode::Char('q') => break,
                KeyCode::Char(c) => input_text.push(c),
                KeyCode::Backspace => {
                    input_text.pop();
                }
                //on entering the enter it adds new line
                KeyCode::Enter => {
                    // Check if the input is "clear"
                    if input_text.trim() == "clear" {
                        // Clear the list
                        alllist.clear();
                        // Optionally clear the input as well
                        input_text.clear();
                    } else if !input_text.trim().is_empty() {
                        // Add the input to the list
                        alllist.push(input_text.clone());
                        // Clear the input after adding
                        input_text.clear();
                    }
                }
                _ => {}
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}
