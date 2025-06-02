// Using Arc for the Thread Safety
use std::sync::Arc;
// Importing thread to run inside the program
use std::thread;

fn main() {
    // Initialize a new arc as it itself is heap allocated value -its immutable
    let v = Arc::new(vec![1, 2, 3]);

    // give v1 a ownership of the arc pointer
    //This does not clone the vector data, just clones the pointer, increasing
    // the reference count to 3 (for v, v1, and v2).
    let v1 = Arc::clone(&v);
    let v2 = Arc::clone(&v);

    let handle1 = thread::spawn(move || println!("Thread 1: {:?}", v1));
    let handle2 = thread::spawn(move || println!("Thread 2: {:?}", v2));

    //wait for threads to complete before the function ends
    handle1.join().unwrap();
    handle2.join().unwrap();

    // Ownership Still  Remains
    println!("{:?}",v)
}
