
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{
         EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
    },
};
use ratatui::{
    prelude::CrosstermBackend,
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem}, Terminal,
};
use walkdir::WalkDir;

struct App {
    selected: usize,
    data: Vec<String>,
}
impl App {
    fn new(data: Vec<String>) -> Self {
        App {
            selected: 0,
            data: data,
        }
    }
    fn next(&mut self) {
        if self.selected < self.data.len() - 1 {
            self.selected += 1;
        }
    }
    fn previous(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        }
    }
}

fn main() -> std::io::Result<()> {
    let filelist: Vec<String> = WalkDir::new("./src")
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
    let app = App::new(filelist);

    let _ = runapp(app, &mut terminal)?;
    disable_raw_mode()?;
    execute!(terminal.backend_mut(),LeaveAlternateScreen)?;
    Ok(())
}

fn runapp<T: ratatui::backend::Backend>(
    mut app: App,
    terminal: &mut Terminal<T>,
) -> std::io::Result<()> {
    loop {
        terminal.draw(|f| {
            let size = f.size();

            let block = Block::default()
                .title("File Structure View")
                .borders(Borders::all());

            let listitem: Vec<ListItem> =
                app.data.iter().map(|x| ListItem::new(x.clone())).collect();
            let list = List::new(listitem)
                .block(block)
                .highlight_style(Style::default().bg(Color::Blue).fg(Color::White))
                .highlight_symbol("-> ");

            let mut list_state = ratatui::widgets::ListState::default();
            list_state.select(Some(app.selected));
            f.render_stateful_widget(list, size, &mut list_state);
        })?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => break,
                KeyCode::Down => app.next(),
                KeyCode::Up => app.previous(),
                //else do nothing
                _ => {}
            }
        }
    }
    Ok(())
}
