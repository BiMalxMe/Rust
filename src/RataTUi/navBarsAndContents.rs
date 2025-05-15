use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    prelude::CrosstermBackend,
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, Borders, Paragraph, Tabs},
    Terminal,
};
use std::io;

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let stdout = io::stdout();
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout))?;
    execute!(terminal.backend_mut(), EnterAlternateScreen)?;

    // vec is &str type in this code

    let nav_titles = ["Home", "Main", "Contact Us", "Help"];
    let mut selected = 0;

    loop {
        terminal.draw(|f| {
            let size = f.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints([Constraint::Length(3), Constraint::Min(0)]) 
                .split(size);

                // changing the &&str to the span for styling
            let tab_titles: Vec<Span> = nav_titles
                .iter()
                .enumerate()
                .map(|(i, t)| {
                    let style = if i == selected {
                        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(Color::White)
                    };
                    //iter return the reference so dereferencing it
                    Span::styled(*t, style)
                })
                .collect();

            let tabs = Tabs::new(tab_titles)
                .select(selected)
                .block(Block::default().title("Tabs").borders(Borders::ALL))
                .highlight_style(Style::default().bg(Color::DarkGray)); // Style for the highlighted tab

            f.render_widget(tabs, chunks[0]);
            let content = match selected {
                0 => "This is the Home page okayy.",
                1 => "Here are your Settings.",
                2 => "Email : bimalcgalise123@gmail.com",
                3 => "Need some assistance? We're here to help!",
                _ => "",
            };

            // create a Paragraph widget to show tab content
            let paragraph = Paragraph::new(content)
                .block(Block::default().borders(Borders::ALL)); // with border

            // render the content in the second chunk
            f.render_widget(paragraph, chunks[1]);
        })?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Left => {
                    if selected > 0 {
                        selected -= 1; // go to previous tab
                    }
                }
                KeyCode::Right => {
                    if selected < nav_titles.len() - 1 {
                        selected += 1; // go to next tab
                    }
                }
                KeyCode::Char('q') => break, // Exit the application on 'q'
                _ => {}
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?; // show the cursor back
    Ok(())
}