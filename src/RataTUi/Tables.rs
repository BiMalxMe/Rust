use ratatui::{
    backend::CrosstermBackend, layout::{Constraint, Direction, Layout}, style::{Color, Modifier, Style}, widgets::{Block, Borders, Row, Table}, Terminal
};
use crossterm::{
    terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    execute, event::{self, Event, KeyCode},
};
use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // turn on raw mode and enter alternate screen
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    loop {
        terminal.draw(|f| {
            // split layout into one vertical chunk
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                // it will take the vertical height accordingly
                .constraints([Constraint::Min(0)])
                .split(f.size());

            // define header and one row
            let header = Row::new(vec!["ID", "Name", "Role"]).style(
                Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
            );
            //divide teh width accordingly
            let width = vec![ Constraint::Length(2),
            Constraint::Length(10),
            Constraint::Length(10),];
            
            //inintalizing the row vector
            let mut rows: Vec<Row> = Vec::new(); 
            rows.push(Row::new(vec!["2", "Bob", "User"]));
rows.push(Row::new(vec!["3", "Charlie", "Guest"]));
rows.push(Row::new(vec!["4", "David", "Admin"]));
            // build table with borders
            let table = Table::new(rows,width)
                .header(header)
                .block(Block::default().title("User Table").borders(Borders::ALL));

                
            // render the table
            f.render_widget(table, chunks[0]);
        })?;

        // press 'q' to quit
        if let Event::Key(key) = event::read()? {
            if key.code == KeyCode::Char('q') {
                break;
            }
        }
    }

    // restore terminal on exit
    disable_raw_mode()?;
    // acessing the EnteredTerminaland Exiting it
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}
