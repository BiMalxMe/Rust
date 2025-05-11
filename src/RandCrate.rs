use rand::{seq::SliceRandom, Rng};  

fn main() {
    let mut rng = rand::thread_rng();  // Get a random number generator
    //accoding to the type of a number is generated

    //(vs code problem) -> Ignore r# for now
    let a: u8= rng.r#gen();  
    println!("Random value is {}", a);
     // Generate a random number between 1 and 100
     let b = rng.gen_range(1..100);
     println!("The random  betn 1 to 100 is {} ",b);

    // Generate a random boolean with a 50% chance of being true
    let c = rng.gen_bool(0.2);
    println!("The Random bool with 20% chance is {} ",c);

      // Generate a random floating-point number in the range [
    let d: f32 = rng.r#gen();
    let e: f64 = rng.gen_range(1.00..100.00);
    println!("The Random floating value is {} ",d);
    println!("The Random floating 1-100 is {} ",e);

     // Create a 16-byte array and fill it with random bytes
     // upto 16 lengths long
     let mut arr = [0u8; 16];
     rng.fill(&mut arr);
     println!("The Random array is  is {:?} ",arr);

     let vec = vec![1, 2, 3, 4, 5];
    
     // Randomly choose one element from the vector
     let sample = vec.choose(&mut rng);
     match sample {
         Some(&value) => println!("Random sample: {}", value),
         None => println!("No elements to sample from"),
     }


}
