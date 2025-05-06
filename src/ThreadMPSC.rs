use std::{
    sync::mpsc,
    thread,
};

fn main() {
    let (tx, rx) = mpsc::channel();

    // using 10 threads for faster coputing
    for i in 0..10 {
        let producer = tx.clone();
        thread::spawn(move || {
            let mut sum : u64 = 0;
            //every time one million calculations
            for j in i * 1000000..(i + 1) * 1000000 {
                sum += j;
            }
            //unwrap expects the result or panics
            producer.send(sum).unwrap();
        });
    }
    drop(tx);
    let mut final_sum = 0;
    // get from multiple threads and combine
    for val in rx {
        final_sum += val;
    }
    println!("{}", final_sum);
}
