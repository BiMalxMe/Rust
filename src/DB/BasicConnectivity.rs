use sqlx::postgres::PgPoolOptions;
use std::env;
use dotenvy::dotenv;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // load .env file
    dotenv().ok(); 

    let database_url = env::var("DATABASE_URL")
     // get database url from environment
        .expect("DATABASE_URL must be set in .env file");

    // create a connection pool with max 5 connections
    let _ = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    println!("✅ connected to the database successfully!");
     // return ok if everything worked
    Ok(())
}
