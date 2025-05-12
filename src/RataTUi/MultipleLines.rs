use std::io::stdout;

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    prelude::CrosstermBackend,
    widgets::{Block, Borders, Paragraph},
    style::{Style, Modifier, Color},  // Using ratatui's Color
    text::{Line, Span},
    Terminal,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout: std::io::Stdout = stdout();
    // shoudl bw used before the backend because it takes mutable borrow
    // but the backedn moves the ownership
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    // make a terminal instance
    let mut terminal = Terminal::new(backend)?;

    // Main loop
    loop {
        terminal.draw(|f| {
            let size = f.size();

            // to use multiple line we should use vec
            let text = vec![
                // to use the span inside line we should use the vector
                //one arg is content and another is styles
                Line::from(vec![Span::styled(
                    "BiMalxMe CommandLine",
                    Style::default()
                        .fg(Color::Cyan)  // ratatui's Color
                        .add_modifier(Modifier::BOLD),
                )]),
                Line::from(""), // Blank line
                Line::from("Press Q to quit."),
                Line::from("Use arrow keys to navigate."),
            ];
            
            // Create a paragraph widget with border and title
            let paragraph = Paragraph::new(text)
                .block(Block::default().title("Instructions").borders(Borders::ALL))
                .style(Style::default().fg(Color::White))
                //Wraps accordingly with txts
                .wrap(ratatui::widgets::Wrap { trim: true });

            // Render the paragraph in the entire terminal size
            f.render_widget(paragraph, size);
        })?;

        // Handle key events
        if let Event::Key(key) = event::read()? {
            if key.code == KeyCode::Char('q') || key.code == KeyCode::Char('Q') {
                break;
            }
            //nothing
        }
    }

//after q is pressed then these code are rendered
    disable_raw_mode()?;
// return back to the original terminal
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}