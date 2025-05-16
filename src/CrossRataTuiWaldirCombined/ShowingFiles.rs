use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{enable_raw_mode, EnterAlternateScreen},
};
use ratatui::{
    prelude::CrosstermBackend,
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem},
    Terminal,
};
use walkdir::WalkDir;

fn main() -> std::io::Result<()> {
    let dirs: Vec<String> = WalkDir::new("./src")
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
        .map(|e| e.path().display().to_string())
        .collect();
    enable_raw_mode()?;

    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    loop {
        terminal.draw(|f| {
            let size = f.size();

            let block = Block::default().title("Directories").borders(Borders::ALL);

            let items: Vec<ListItem> = dirs.iter().map(|d| ListItem::new(d.clone())).collect();
            let list = List::new(items)
                .block(block)
                .style(Style::default().fg(Color::White));

            f.render_widget(list, size);
        })?;
        if let Event::Key(key) = event::read()? {
            if key.code == KeyCode::Char('q') {
                break;
            }
        }
    }

    Ok(())
}