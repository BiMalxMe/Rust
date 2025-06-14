use sqlx::postgres::PgPoolOptions;
use dotenvy::dotenv;
use std::env;
use tokio;
use sqlx::Row;

use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Gauge},
};
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{io, thread, time::Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup terminal UI
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut progress = 0.0;
    while progress <= 1.0 {
        terminal.draw(|f| {
            let size = f.size();
            let gauge = Gauge::default()
                .block(Block::default().title("Connecting to DB...").borders(Borders::ALL))
                .gauge_style(Style::default().fg(Color::Cyan).bg(Color::Black))
                .ratio(progress);
            f.render_widget(gauge, size);
        })?;

        progress += 0.05;
        thread::sleep(Duration::from_millis(50));
    }

    // Restore terminal before DB connection
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;

    // Load environment variables from .env file
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");

    // Connect to PostgreSQL
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    println!("\n✅ Connected to database");

    // Setup and insert
    let setup_and_insert_query = r#"
        create table if not exists users (
            id serial primary key,
            username text not null,
            email text unique not null
        );

        insert into users (username, email)
        values ('naman', 'sch@example.com')
        on conflict (email) do nothing;
    "#;

    for query in setup_and_insert_query.split(';') {
        if query.trim().is_empty() {
            continue;
        }
        sqlx::query(query).execute(&pool).await?;
    }

    println!("\n📦 Table created and user inserted (if not already)");

    // Fetch and print all users
    let rows = sqlx::query("select id, username, email from users")
        .fetch_all(&pool)
        .await?;

    println!("\n📋 All users:");
    for row in rows {
        let id: i32 = row.get("id");
        let username: String = row.get("username");
        let email: String = row.get("email");

        println!("👉 id: {}, username: {}, email: {}", id, username, email);
    }

    Ok(())

}
