use std::io::stdout;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen };
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

        let mainchuncks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(50),
                Constraint::Percentage(50),
                ]).split(size);

        let chunkstop = Layout::default()
        //vertically splitting into two boxes vertically
            .direction(Direction::Horizontal)

            // diving 50-50%
            .constraints([
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ])
            //split by current size
            .split(mainchuncks[0]); 
        
        //render something in that space
        let chunksdown = Layout::default()
        //vertically splitting into two boxes vertically
            .direction(Direction::Horizontal)

            // diving 50-50%
            .constraints([
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ])
            //split by current size
            .split(mainchuncks[1]); 
        
            f.render_widget(
                Paragraph::new("Top")
                    .block(Block::new()
                        .borders(Borders::ALL)
                        .title("Hello")),  // Adding title to the block
                chunkstop[0]
            );
        f.render_widget(
            Paragraph::new("Bottom")
                .block(Block::new().borders(Borders::ALL)),
                chunkstop[1]);
        
        f.render_widget(
            Paragraph::new("Top")
                .block(Block::new().borders(Borders::ALL)),
                chunksdown[0]);
        
        f.render_widget(
            Paragraph::new("Bottom")
                .block(Block::new().borders(Borders::ALL)),
                chunksdown[1]);
    }).unwrap();
    
    disable_raw_mode()?;
    Ok(())
}