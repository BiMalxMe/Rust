use sqlx::{postgres::PgPoolOptions, PgPool};
use dotenvy::dotenv;
use std::{env, io::stdin};
use chrono::NaiveDateTime;

struct NewTodo {
    title: String,
    description: String,
    deadline: Option<NaiveDateTime>,
}

struct Todo;

impl Todo {
    /// Create the 'todo' table if it doesn't exist
    async fn create_table(pool: &PgPool) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS todo (
                id SERIAL PRIMARY KEY,
                title TEXT NOT NULL,
                description TEXT NOT NULL,
                completed BOOLEAN NOT NULL DEFAULT FALSE,
                deadline TIMESTAMP NULL
            )
            "#
        )
        .execute(pool)
        .await?;
        Ok(())
    }

    /// Insert a new todo into the database
    async fn insert(pool: &PgPool, new_todo: NewTodo) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO todo (title, description, completed, deadline)
            VALUES ($1, $2, FALSE, $3)
            "#
        )
        .bind(&new_todo.title)
        .bind(&new_todo.description)
        .bind(&new_todo.deadline) // Now correctly using NaiveDateTime
        .execute(pool)
        .await?;
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    println!("Connected to the database");

    Todo::create_table(&pool).await?;
    println!("'todo' table ensured to exist.");

    println!("\n--- Add New Todo ---");

    println!("Enter title");
    let mut title = String::new();
    stdin().read_line(&mut title).expect("Failed to read title");
    let title = title.trim().to_string();

    println!("Enter description:");
    let mut description = String::new();
    stdin().read_line(&mut description).expect("Failed to read description");
    //trim returns the &str while trimming so we use to_string()
    let description = description.trim().to_string();

    println!("Enter deadline (YYYY-MM-DD HH:MM:SS, optional, press Enter to skip)");
    let mut deadline_str = String::new();
    stdin().read_line(&mut deadline_str).expect("Failed to read deadline");
    
    //can also be empty so 
    let deadline = if deadline_str.trim().is_empty() {
        None
    } else {
        match NaiveDateTime::parse_from_str(deadline_str.trim(), "%Y-%m-%d %H:%M:%S") {
            Ok(dt) => Some(dt),
            Err(_) => {
                println!("Invalid deadline format. Skipping deadline.");
                None
            }
        }
    };

    let new_todo = NewTodo { title, description, deadline };

    if let Err(e) = Todo::insert(&pool, new_todo).await {
        eprintln!("Error adding todo: {}", e);
    } else {
        println!("Todo added successfully!");
    }

    println!("\nPress Enter to exit...");
    let mut _dummy = String::new();
    stdin().read_line(&mut _dummy).expect("Error reading line");

    Ok(())
}
