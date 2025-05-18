// Import standard I/O module
use std::io;

use crossterm::{
    event::{self, Event, KeyCode}, // For capturing events and specific key codes
    execute,                        // For executing terminal commands
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}, // Terminal mode control
};

// Import required modules from the ratatui crate for TUI rendering
use ratatui::{
    backend::CrosstermBackend,      // backend adapter to connect Ratatui with Crossterm
    Terminal,                       // main interface to draw UI
    widgets::{Block, Borders, List, ListItem}, // Widgets used to build UI components
    style::{Color, Style},          // Styling for UI elements
};

fn main() -> io::Result<()> {
    // Create a vector of strings: "Item 1" to "Item 20"
    let items: Vec<String> = (1..=20).map(|i| format!("Item {}", i)).collect();

    // Enable raw mode so input can be read instantly (without Enter)
    enable_raw_mode()?;

    // Get a handle to stdout for terminal operations
    let mut stdout = io::stdout();

    // Switch to alternate screen buffer (keeps main terminal untouched)
    execute!(stdout, EnterAlternateScreen)?;

    // Create a Crossterm backend for Ratatui to use
    let backend = CrosstermBackend::new(stdout);

    // Create a terminal object to draw UI using the backend
    let mut terminal = Terminal::new(backend)?;

    // State variables: current selected item and scroll offset
    let mut selected = 0;
    let mut scroll_offset = 0;

    loop {
        // Render UI
        terminal.draw(|f| {
            // Get terminal dimensions
            let size = f.size();

            // Compute how many items can be displayed (excluding 2 lines for border)
            let max_visible = size.height.saturating_sub(2) as usize;

            // Get the slice of items currently visible based on scroll_offset
            let visible_items = items
                .iter()
                // skip up to the scroll offset
                .skip(scroll_offset)
                //take how many list after the scrolled offset
                .take(max_visible)
                // change every element of items into a ListItem
                .map(|i| ListItem::new(i.clone())) // Convert to ListItem
                .collect::<Vec<_>>(); // Collect into a Vec<ListItem>

            // Create a bordered block with title
            let list = List::new(visible_items)
                .block(Block::default().title("q to quit").borders(Borders::ALL))
                .highlight_style(Style::default().bg(Color::Blue).fg(Color::White)) // Style for selected item
                .highlight_symbol(">> "); // Prefix for selected item

            // Set the currently selected item in list state
            let mut state = ratatui::widgets::ListState::default();
            state.select(Some(selected - scroll_offset)); // Adjust for scrolling

            // Render the list with selection state
            f.render_stateful_widget(list, size, &mut state);
        })?;

        // Handle user input
        if let Event::Key(key) = event::read()? {
            match key.code {
                // Quit on 'q' key
                KeyCode::Char('q') => break,

                // Move selection down
                KeyCode::Down => {
                    if selected < items.len() - 1 {
                        selected += 1;

                        // Scroll down if selection goes beyond visible items
                        if selected >= scroll_offset + terminal.size()?.height as usize - 2 {
                            scroll_offset += 5; // Scroll by 5 items at a time
                        }
                    }
                }

                // Move selection up
                KeyCode::Up => {
                    if selected > 0 {
                        selected -= 1;

                        // Scroll up if selection is above visible range
                        if selected < scroll_offset {
                            scroll_offset -= 1;
                        }
                    }
                }

                // Ignore other keys
                _ => {}
            }
        }
    }

    //  disable raw mode
    disable_raw_mode()?;

    // Leave the alternate screen and return to normal screen
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

    // Show the cursor again (it is hidden by default in raw mode)
    terminal.show_cursor()?;

    // Return success
    Ok(())
}
