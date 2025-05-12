use std::io::stdout;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen};
use crossterm::ExecutableCommand;
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::{backend, layout::Layout, Terminal};
//simple layout import
use ratatui::prelude::*;

fn main() -> std::io::Result<()>{
    enable_raw_mode()?;
    let mut stdout = stdout();
    //Changing the terminal
    stdout.execute(EnterAlternateScreen)?;
    let backend = backend::CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal.draw(|f| {
        let size = f.size();

        let chunks = Layout::default()
        //vertically splitting into two boxes vertically
            .direction(Direction::Vertical)

            // diving 50-50%
            .constraints([
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ])
            //split by current size
            .split(size); 
        
        //render something in that space
        f.render_widget(
            Paragraph::new("Top")
                .block(Block::new().borders(Borders::ALL)),
            chunks[0]);
        
        f.render_widget(
            Paragraph::new("Bottom")
                .block(Block::new().borders(Borders::ALL)),
            chunks[1]);
    }).unwrap();
    
    disable_raw_mode()?;
    Ok(())
}