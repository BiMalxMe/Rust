use sqlx::postgres::PgPoolOptions;
use dotenvy::dotenv;
use std::env;
use tokio;

// Row is needed to extract values from query results
use sqlx::Row; 

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // load environment variables from .env file
    dotenv().ok();

    // get the database url from environment
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env");

    // create a postgres connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    println!("✅ connected to database");

    // create users table if it doesn't exist
    // insert a user (skip if email exists)
    // then fetch and print all users
    let setup_and_insert_query = r#"
        create table if not exists users (
            id serial primary key,
            username text not null,
            email text unique not null
        );

        insert into users (username, email)
        values ('bimal', 'bml@example.com')
        on conflict (email) do nothing;
    "#;

    // execute both create and insert in one go
    for query in setup_and_insert_query.split(';') {
        if query.trim().is_empty() { continue; }
        sqlx::query(query).execute(&pool).await?;
    }

    println!(" Table created and user inserted (if not created earlier) and data is inserted ");

    // fetch all users from table
    let rows = sqlx::query("select id, username, email from users")
    //fetching the datas by passing the query in the database
        .fetch_all(&pool)
        .await?;

    println!(" all users are "  );
    for row in rows {
        let id: i32 = row.get("id");
        let username: String = row.get("username");
        let email: String = row.get("email");

        println!("id: {}, username: {}, email: {}", id, username, email);
    }

    Ok(())
}
