use sqlx::postgres::PgPoolOptions;
use dotenvy::dotenv;
use std::env;
use tokio;
use sqlx::Row; // required for `.get()`

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // load the .env file
    dotenv().ok();

    // get the DATABASE_URL from the environment
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env");

    // connect to the database
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    println!("✅ connected to database");

    // create the table if not exists
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id SERIAL PRIMARY KEY,
            username TEXT NOT NULL,
            email TEXT UNIQUE NOT NULL
        )
        "#
    )
    .execute(&pool)
    .await?;

    println!("✅ table created (if it didn't exist)");

    // insert a new user (skip if email already exists)
    let username = "bimal";
    let email = "bml@example.com";

    sqlx::query(
        r#"
        INSERT INTO users (username, email)
        VALUES ($1, $2)
        ON CONFLICT (email) DO NOTHING
        "#
    )
    .bind(username)
    .bind(email)
    .execute(&pool)
    .await?;

    println!("✅ user inserted (if not already exists)");

    // fetch and display all users
    let rows = sqlx::query(
        r#"
        SELECT id, username, email FROM users
        "#
    )
    .fetch_all(&pool)
    .await?;

    println!(" all users:");
    for row in rows {
        let id: i32 = row.get("id");
        let username: String = row.get("username");
        let email: String = row.get("email");

        println!("id: {}, username: {}, email: {}", id, username, email);
    }

    Ok(())
}
