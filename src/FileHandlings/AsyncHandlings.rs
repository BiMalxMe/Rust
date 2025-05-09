// Doing the asynchronous file handles

use tokio::fs;
use tokio::time::Instant;

#[tokio::main]
async fn main() -> tokio::io::Result<()>{
    let time = Instant::now();
    let write1 = fs::write("bimal.txt", "Async file 1").await;
    let write2 = fs::write("bimal2.txt", "Async file 2").await;
    write1?;
    write2?;
    let totaltime = time.elapsed();
    println!("Written Synchronously {:?}",totaltime);
    Ok(())
}
