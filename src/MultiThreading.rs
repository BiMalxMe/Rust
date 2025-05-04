//Importing this for thread related thingee
use std::thread;
//for using the sleep keyword
use std::time::Duration;

//Both the main and the spawned thread are running at a same time

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..200 {
            println!("{i} 🧵 From the Spawned Thread");
            //sleepint this thread such that the main thread also can get the time
            thread::sleep(Duration::from_millis(10));
        }
    });

    let mut sum = 0;
    for i in 1..1000 {
        sum += i;
        println!("The iteration in main thread {i}");
                    //sleepint this thread such that the spawned thread also can get the time
        thread::sleep(Duration::from_millis(10)); // Yield time to spawned thread
    }

    //at last after both spawned and 99 percent main therad is complete.
    println!("Sum is {sum}");
    //unwrap() is used to get the result of join() and will crash the program if there's an error.
    handle.join().unwrap(); // Wait for the spawned thread to finish
}
