// Import Mutex to enable safe mutable access to shared data with locking.
use std::sync::{Arc, Mutex};

//Import thread module to spawn new threads.
use std::thread;

fn main() {
    // Wrap the vector inside a Mutex — this allows safe mutable access with locking.
    //Wrap the Mutex inside an Arc — this allows multiple
    // threads to share ownership of the same Mutex and vector.
    let v = Arc::new(Mutex::new(vec![1, 2, 3]));
    
    //This does not clone the vector data, just clones the pointer, increasing
    // the reference count to 3 (for v, v1, and v2).
    let v1 = Arc::clone(&v);
    let v2 = Arc::clone(&v);

    //move the arc pointers values into the handlw1
    let handle1 = thread::spawn(move || {
        // lock gives acess to get the /mutable elements
        // and block other thread to use it until it is using
        //lock make sures when it runs it get acess and modify the vec
        let mut vec = v1.lock().unwrap();  

        //push into the original vec
        vec.push(4);
        // *vec means: "give me the actual Vec<i32> inside the guard"
        println!("Thread 1: {:?}", *vec);
    });

    let handle2 = thread::spawn(move || {
        let mut vec = v2.lock().unwrap();
        vec.push(5);
        println!("Thread 2: {:?}", *vec);
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    // Accessing the vector again after threads have joined
    println!("Main thread: {:?}", *v.lock().unwrap());
}
