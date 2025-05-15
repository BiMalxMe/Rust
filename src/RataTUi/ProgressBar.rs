use std::{ io, thread, time::Duration};

use crossterm::{ execute, terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}};
use ratatui::{layout::{Constraint, Direction, Layout}, prelude::CrosstermBackend, style::{Color, Style}, widgets::{Block, Borders, Gauge}, Terminal};

fn main() -> io::Result<()>{
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend =  CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut percent : f32 = 0.00;

    loop {
        terminal.draw( |f| {
            let size = f.size();

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints([Constraint::Length(3)])
                .split(size); // Use the 'size' variable

            let gauge = Gauge::default()
            .block(Block::default().title("Progress Bar").borders(Borders::all()))
            .gauge_style(Style::default().fg(Color::Green))
            // convert float to percentage
            // Making Minimum percentage 100 such that there is no error occurring
            .percent((percent * 100.0).min(100.0) as u16);

            f.render_widget(gauge, chunks[0]);
        })?;

        if percent >= 1.00 {
            execute!(io::stdout(), LeaveAlternateScreen)?;
            disable_raw_mode()?;
            terminal.show_cursor()?;
            return Ok(());
        }

        percent += 0.3;
        thread::sleep(Duration::from_millis(300));
    }
}