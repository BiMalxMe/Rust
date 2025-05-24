use std::io;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    widgets::{Block, Borders, Paragraph},
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    Terminal,
};

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut show_dialog = false;

    loop {
        terminal.draw(|f| {
            let size = f.size();

            // Layout
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(9)
                .constraints([Constraint::Percentage(70)])
                .split(size);

            // Main content
            let main_block = Block::default().title("Main UI").borders(Borders::ALL);
            f.render_widget(main_block, size);

            // Conditional dialog
            if show_dialog {
                let dialog = Paragraph::new("This is a dialog box!\nPress D again to close.")
                    .style(Style::default().bg(Color::Black).fg(Color::White))
                    .alignment(Alignment::Center)
                    .block(Block::default().title("Dialog").borders(Borders::ALL));
                f.render_widget(dialog, chunks[0]);
            }
        })?;

        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('d') => {
                        show_dialog = !show_dialog;
                    }
                    KeyCode::Char('q') => {
                        break;
                    }
                    _ => {}
                }
            }
        }
    }

    // Cleanup
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}
