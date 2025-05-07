use tokio::time::{sleep, Duration};
//import the duration and sleep to sleep the user for the certain duration

// Use this to use the async in the main function
#[tokio::main]
async fn main() {
    say_hello().await;
}

//defineing the async function say_hello()
async fn say_hello() {
    println!("Hello...");
    
    // Program is awaited here for two seconds
    sleep(Duration::from_secs(2)).await;
    println!("...World!");
}
