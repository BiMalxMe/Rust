use ratatui::{
    backend::CrosstermBackend, layout::{Constraint, Direction, Layout}, style::{Color, Style}, symbols:: Marker, widgets::{Axis, Block, Borders, Chart, Dataset, GraphType}, Terminal
};
use crossterm::{
    execute,
    terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    event::{self, Event, KeyCode},
};
use std::{io, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    // enable raw mode for terminal control
    enable_raw_mode()?;
    let mut stdout = io::stdout();

    // switch to alternate screen
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints([Constraint::Min(0)])
                .split(f.size());
            let data = [
                (12.34, 87.65), (98.76, 23.45), (45.67, 56.78), (78.90, 32.10), (23.45, 98.76),
                (65.43, 43.21), (32.10, 78.90), (87.65, 12.34), (56.78, 45.67), (44.44, 77.77),
                (77.77, 44.44), (11.11, 99.99), (99.99, 11.11), (22.22, 88.88), (88.88, 22.22),
                (33.33, 66.66), (66.66, 33.33), (55.55, 55.55), (10.10, 20.20), (30.30, 40.40),
                (50.50, 60.60), (70.70, 80.80), (90.90, 10.00), (2.00, 91.90), (81.80, 12.10),
                (21.20, 89.80), (98.90, 21.20), (19.10, 90.90), (72.70, 38.30), (38.30, 72.70),
                (41.40, 59.50), (59.50, 41.40), (62.60, 37.30), (37.30, 62.60), (13.10, 86.80),
                (86.80, 13.10), (74.70, 25.20), (25.20, 74.70), (48.40, 51.50), (51.50, 48.40),
                (92.90, 7.00), (7.00, 92.90), (35.30, 64.60), (64.60, 35.30), (71.70, 28.20),
                (28.20, 71.70), (53.50, 46.40), (46.40, 53.50), (84.80, 15.10), (15.10, 84.80),
                (95.90, 4.00), (4.00, 95.90), (31.30, 68.60), (68.60, 31.30), (76.70, 23.20),
                (23.20, 76.70), (57.50, 42.40), (42.40, 57.50), (88.80, 11.10), (11.10, 88.80),
                (99.00, 0.90), (0.90, 99.00), (39.30, 60.60), (60.60, 39.30), (73.70, 26.20),
                (26.20, 73.70), (52.50, 47.40), (47.40, 52.50), (80.80, 19.10), (19.10, 80.80),
                (91.90, 8.00), (8.00, 91.90), (34.30, 65.60), (65.60, 34.30), (77.70, 22.20),
                (22.20, 77.70), (58.50, 41.40), (41.40, 58.50), (85.80, 14.10), (14.10, 85.80),
                (96.90, 3.00), (3.00, 96.90), (30.30, 69.60), (69.60, 30.30), (79.70, 20.20),
                (20.20, 79.70), (54.50, 45.40), (45.40, 54.50), (82.80, 17.10), (17.10, 82.80),
                (93.90, 6.00), (6.00, 93.90), (36.30, 63.60), (63.60, 36.30), (70.70, 29.20),
                (29.20, 70.70), (50.50, 49.40), (49.40, 50.50), (87.80, 12.10), (12.10, 87.80),
                (97.90, 2.00), (2.00, 97.90), (32.30, 67.60), (67.60, 32.30), (75.70, 24.20)
            ];
          
          let dataset = Dataset::default()
          .data(&data)
          .graph_type(GraphType::Line)
          .marker(Marker::Bar)
          .style(Style::default().fg(Color::Blue));

        let chart = Chart::new(vec![dataset]) 
        .block(Block::default().title("BimalsData").borders(Borders::all()))
        .x_axis(Axis::default().title("Xaxis").bounds([1.0,100.0]).style(Style::default().fg(Color::Gray)))
        .y_axis(Axis::default().title("Yaxis").bounds([1.0,100.0]).style(Style::default().fg(Color::Gray)));

    f.render_widget(chart, chunks[0]);
        })?;

        // Press 'q' to quit
        if let Event::Key(key) = event::read()? {
            if key.code == KeyCode::Char('q') {
                break;
            }
        }
    }

    // cleanup terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}
