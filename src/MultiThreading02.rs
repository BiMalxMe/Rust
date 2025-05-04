//Importing this for thread related thingee
use std::thread;
//for using the sleep keyword
use std::time::Duration;

//Both the main and the spawned thread are running at a same time

fn main() {
    let handle1 = thread::spawn(|| {
        
        println!(" 🧵 From the Spawned Thread handle1");
        thread::sleep(Duration::from_millis(10));
    
});
  
    let handle2 = thread::spawn(|| {
        
            println!(" 🧵 From the Spawned Thread handle2");
            thread::sleep(Duration::from_millis(10));
        
    });
    let handle3 = thread::spawn(|| {
        
        println!(" 🧵 From the Spawned Thread handle3");
        thread::sleep(Duration::from_millis(10));
    
});
let handle4 = thread::spawn(|| {
        
    println!(" 🧵 From the Spawned Thread handle4");
    thread::sleep(Duration::from_millis(10));

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
    handle1.join().unwrap();
    handle2.join().unwrap();
    handle3.join().unwrap();
    handle4.join().unwrap();}


    //result

//     Running `target/debug/rust-proj`
//     🧵 From the Spawned Thread handle1
//     🧵 From the Spawned Thread handle4
//     🧵 From the Spawned Thread handle3
//    The iteration in main thread 1
//     🧵 From the Spawned Thread handle2
//    The iteration in main thread 2
//    The iteration in main thread 3
//    The iteration in main thread 4
//    The iteration in main thread 5
//    The iteration in main thread 6
//    The iteration in main thread 7
//    The iteration in main thread 8
//    The iteration in main thread 9
//    The iteration in main thread 10
//    The iteration in main thread 11
//    The iteration in main thread 12
//    The iteration in main thread 13
//    The iteration in main thread 14
//    The iteration in main thread 15
//    The iteration in main thread 16
//    The iteration in main thread 17
//    The iteration in main thread 18
//    The iteration in main thread 19
//    The iteration in main thread 20
//    The iteration in main thread 21
//    The iteration in main thread 22
//    The iteration in main thread 23
//    The iteration in main thread 24
//    The iteration in main thread 25
//    The iteration in main thread 26
//    The iteration in main thread 27
//    The iteration in main thread 28
//    The iteration in main thread 29..........
