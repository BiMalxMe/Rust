use tokio::time::{sleep, Duration};
use tokio::join;

#[tokio::main]
async fn main() {
    // Spawn Task 1: sleeps for 1 second, then prints a message
    let task1 = tokio::spawn(async {
        sleep(Duration::from_secs(1)).await;
        println!("Task 1 done!");
    });

    // Spawn Task 2: sleeps for 2 seconds, then prints a message
    let task2 = tokio::spawn(async {
        sleep(Duration::from_secs(2)).await;
        println!("Task 2 done!");
    });

    // Spawn Task 3: sleeps for 3 seconds, then prints a message
    let task3 = tokio::spawn(async {
        sleep(Duration::from_secs(3)).await;
        println!("Task 3 done!");
    });

    // Use `join!` to wait for all three tasks to complete concurrently.
    // `join!` returns when ALL futures passed to it are done.
     let _=join!(task1, task2, task3);

    // This line runs only after all tasks above have completed
    println!("All tasks completed.");
}
g