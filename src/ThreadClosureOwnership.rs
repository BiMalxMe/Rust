// Closures and the thread some thing


use std::thread;
 
 //                 PROBLEM
fn main1() {
    //Initalizing the vector
let v = vec![1,2,3];
//this does not compiles because
//      1. The main may end before the spwaned thread
//      2. That may cause the dangling reference
    thread::spawn(||{
        println!("The vec is {:?}",v);
    });
}

//                              FIX


fn main() {
    //Initalizing the vector
let v = vec![1,2,3];
//using the move closure such that the ownership is transmitted as the vector can be there until
// the spawned thread exists
//We need to use move to get the surrounding vars 
    thread::spawn(move ||{
        println!("The vec is {:?}",v);
    });
}