# LogsGenerator: Simple Asynchronous Log File Writer in Rust

[![Screen Shot 2025-05-09 at 12 39 54](https://github.com/user-attachments/assets/75432324-807e-444d-ab79-e71643b63a9b)](https://github.com/user-attachments/assets/75432324-807e-444d-ab79-e71643b63a9b)
[![Screen Shot 2025-05-09 at 12 39 48](https://github.com/user-attachments/assets/fc495531-8430-47ec-96e5-8682e0460efb)](https://github.com/user-attachments/assets/fc495531-8430-47ec-96e5-8682e0460efb)

This Rust application, `LogsGenerator`, demonstrates a basic asynchronous file writing process using the `tokio` runtime. It concurrently writes simple messages to two separate files: `main.txt` and `log.log`. The `log.log` file includes timestamps for each entry, providing a basic logging mechanism.

## Code Overview

The core logic of the application resides in the `src/practice/LogsGenerator` directory (based on the output you provided). Here's a breakdown of the key components:

```rust
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt;
use chrono::Local;

#[tokio::main]
async fn main() -> tokio::io::Result<()>{
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("main.txt").await?;
    let mut log = OpenOptions::new()
        .append(true)
        .create(true)
        .open("log.log").await?;
    for i in 1..100{
        let current = Local::now();
        let time = current.date_naive();
        let date = current.time();
        file.write_all(format!("hello brother {}\n", i).as_bytes()).await?;
        let stringformat = format!(" [{}] [{}] -> Log.{}. Appended Succssfully \n",date,time,i);
         log.write_all(stringformat.as_bytes()).await?;
}
    Ok(())
}
