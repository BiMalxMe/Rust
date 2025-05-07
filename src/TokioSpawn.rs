use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    //spawn runs directly but the normal function is run once it is called
    let handle = tokio::spawn(async {
        println!("Doing async work...");
        sleep(Duration::from_secs(2)).await;
        println!("Done async!");
    });
    //main and the spawned async runs concurrently

    println!("Before awaiting handle..."); //printed right away
    
    handle.await.unwrap(); // wait for the async task to finish
    
    println!("After awaiting handle...");
}
