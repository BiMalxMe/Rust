use ratatui::{
    backend::CrosstermBackend,
    Terminal,
    widgets::{Block, Borders, List, ListItem, ListState},
    layout::{Layout, Constraint, Direction},
    style::{Style, Color, Modifier},
};
use crossterm::{
    terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    execute,
    event::{self, Event, KeyCode},
};
use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Enable raw mode (captures input instantly, disables line buffering)
    enable_raw_mode()?;

    // Switch to alternate screen (UI doesn't overwrite main terminal)
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    // Set up terminal backend
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // List state to track selected item index
    let mut list_state = ListState::default();
    let items = vec![
        ListItem::new("Item 1"),
        ListItem::new("Item 2"),
        ListItem::new("Item 3"),
        ListItem::new("Item 4"),
    ];
    let mut selected = 0;
    list_state.select(Some(selected));

    // Main UI loop
    loop {
        terminal.draw(|f| {
            // Create layout with one full-size vertical chunk
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([Constraint::Min(0)])
                .split(f.size());

            // Create the interactive list widget
            let list = List::new(items.clone())
                .block(Block::default().title("Interactive List").borders(Borders::ALL))
                .highlight_style(
                    Style::default()
                        .bg(Color::Blue) // Background for selected item
                        .fg(Color::White) // Text color for selected item
                        .add_modifier(Modifier::BOLD), // Make it bold
                );

            // Render list with selection state
            // &mut list_state.clone() -> mutable reference to the ListState, which holds the currently selected item.
            f.render_stateful_widget(list, chunks[0], &mut list_state.clone());
        })?;

        // Handle key events
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => break, // Quit on 'q'
                KeyCode::Down => {
                    // Move selection down
                    selected = (selected + 1) % items.len();
                    list_state.select(Some(selected));
                }
                KeyCode::Up => {
                    // Move selection up
                    selected = (selected + items.len() - 1) % items.len();
                    list_state.select(Some(selected));
                }
                //b select garda third index ko lai select garne
                KeyCode::Char('b') => {
                    list_state.select(Some(2));
                }
                _ => {}
            }
        }
    }

    // Restore terminal state before exiting
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}
