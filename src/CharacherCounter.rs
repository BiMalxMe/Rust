use std::collections::HashMap;
use std::io;

fn main() {
    let mut newdataraw = String::new();

    // Prompt the user to enter a string
    println!("Enter any string : ");
    io::stdin()
        .read_line(&mut newdataraw)  // Read input from standard input
        .expect("Failed to read line");  // Handle potential input errors

    let newdata = newdataraw.trim(); // Trim leading/trailing whitespace
    let mut user: HashMap<char, i32> = HashMap::new(); // Create a mutable HashMap

    for ch in newdata.chars() {
        // This filters out anything that’s not a letter.
        if ch.is_alphabetic() {
            // Returns the lowercase version of that 
            // character if it's an ASCII uppercase letter (like 'A' → 'a').
            let lowercase = ch.to_ascii_lowercase();

            // Insert character into HashMap if not already present, starting at 0
            let counter = user.entry(lowercase).or_insert(0);

            // Increment the character count
        *counter += 1;  
      }
    }

    // Print the final character frequency count
    println!("Character frequency count:");
    for (key, value) in &user {
        println!("'{}': {}", key, value);
    }
}
