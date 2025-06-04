use sqlx::postgres::PgPoolOptions;
use dotenvy::dotenv;
use std::env;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // Load the .env file
    dotenv().ok();

    // Get the DATABASE_URL from the environment
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env");

    // Connect to the PostgreSQL database
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    println!("✅ Connected to database");

    // Create the table if it doesn't exist
    //r#""# is a raw string that allows newlines and quotes easily.
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

    println!("✅ Table created (if it didn't exist)");

    // Insert a new user
    let username = "alice";
    let email = "alice@example.com";

            // If the email already exists, skip the insert (no error).

    sqlx::query(
        r#"
        INSERT INTO users (username, email)
        VALUES ($1, $2)
        ON CONFLICT (email) DO NOTHING
        "#
    )

    // Bind help to protect from the SQL injections
    // $1 --> firstbind
    // $2 --> secondBind
    .bind(username)
    .bind(email)
    .execute(&pool)
    .await?;

    println!("✅ User inserted (if not already exists)");

    Ok(())
}
