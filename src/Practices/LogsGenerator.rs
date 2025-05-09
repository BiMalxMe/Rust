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
