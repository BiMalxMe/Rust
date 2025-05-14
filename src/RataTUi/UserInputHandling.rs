use ratatui::{
    backend::CrosstermBackend,
    Terminal,
    widgets::{Paragraph, Block, Borders},
    layout::{Layout, Constraint, Direction},
    text::{Span, Line},
};
use crossterm::{
    execute,
    event::{self, Event, KeyCode},
    terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // enable raw mode and switch to alternate screen
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut input_text = String::new(); // stores typed text

    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(0)])
                .margin(1)
                .split(f.size());

            // show the typed text
            let paragraph = Paragraph::new(Line::from(Span::raw(&input_text)))
                .block(Block::default().title("Type Something").borders(Borders::ALL));

            f.render_widget(paragraph, chunks[0]);
        })?;

        // read key event
        if let Event::Key(key_event) = event::read()? {
            match key_event.code {
                KeyCode::Char('q') => break, // quit on 'q'
                KeyCode::Char(c) => input_text.push(c), // add character
                KeyCode::Backspace => {
                    input_text.pop(); // remove last character
                }
                _ => {}
            }
        }
    }

    // restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}
