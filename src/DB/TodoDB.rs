use sqlx::{postgres::PgPoolOptions, PgPool,Row};
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
        // Now correctly using NaiveDateTime
        .bind(&new_todo.deadline) 
        .execute(pool)
        .await?;
        Ok(())
    }
     async fn view_all(pool: &PgPool) -> Result<(), sqlx::Error> {
        let rows = sqlx::query(
            r#"
            SELECT id, title, description, completed, deadline
            FROM todo
            ORDER BY id
            "#
        )
        .fetch_all(pool)
        .await?;

        if rows.is_empty() {
            println!("No todos found.");
        } else {
            println!("\n--- All Todos ---");
            for row in rows {
                let id: i32 = row.get("id");
                let title: String = row.get("title");
                let description: String = row.get("description");
                let completed: bool = row.get("completed");
                let deadline: Option<NaiveDateTime> = row.get("deadline");

                println!("ID: {}", id);
                println!("Title: {}", title);
                println!("Description: {}", description);
                println!("Completed: {}", completed);
                match deadline {
                    Some(d) => println!("Deadline: {}", d),
                    None => println!("Deadline: None"),
                }
                println!("-------------------------");
            }
        }

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

    println!("\nChoose an action:");
    println!("1. Add New Todo");
    println!("2. View All Todos");
    println!("Enter your choice (1 or 2):");

    let mut choice = String::new();
    stdin().read_line(&mut choice).expect("Failed to read input");
    let choice = choice.trim();

    match choice {
        "1" => {
            println!("Enter title:");
            let mut title = String::new();
            stdin().read_line(&mut title).expect("Failed to read title");
            let title = title.trim().to_string();

            println!("Enter description:");
            let mut description = String::new();
            stdin().read_line(&mut description).expect("Failed to read description");
            let description = description.trim().to_string();

            println!("Enter deadline (YYYY-MM-DD HH:MM:SS, optional, press Enter to skip):");
            let mut deadline_str = String::new();
            stdin().read_line(&mut deadline_str).expect("Failed to read deadline");

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

            match Todo::insert(&pool, new_todo).await {
                Ok(_) => println!("Todo added successfully!"),
                Err(e) => eprintln!("Error adding todo: {}", e),
            }
        }

        "2" => {
            if let Err(e) = Todo::view_all(&pool).await {
                eprintln!("Error fetching todos: {}", e);
            }
        }

        _ => {
            println!("Invalid choice.");
        }
    }

    println!("\nPress Enter to exit...");
    let mut _dummy = String::new();
    stdin().read_line(&mut _dummy).expect("Error reading line");

    Ok(())
}


//sqlx = { version = "0.7", features = ["postgres", "runtime-tokio-native-tls", "chrono"] }

// This tells Cargo:

// "Give me SQLx version 0.7, and I’ll be working with:

// PostgreSQL databases,

// the Tokio runtime with native TLS,

// and I want to use chrono datetime types in my queries."