// Import the Serialize and Deserialize traits from the serde crate.
// These traits allow a struct to be converted to/from data formats (like JSON).
use serde::{Serialize, Deserialize};

// Derive macros automatically generate implementations for the traits:
// - Serialize: to convert the struct into a serial format (e.g., JSON)
// - Deserialize: to convert JSON (or another format) into the struct
// - Debug: allows using {:?} to print the struct
#[derive(Serialize, Deserialize, Debug)]
struct User {
    username: String, 
    age: u8,          
}

fn main() {
    // Create a new instance of the User struct
    let user = User {
        username: "DrBimal".to_string(), // Convert string literal to String
        age: 28,                         // Set age to 28
    };

    // change the stuct into a json format
    let json = serde_json::to_string(&user).unwrap();
    // print the json string
    println!("Serialized: {}", json);

    // return back to other from the json
    let deserialized :User = serde_json::from_str(&json).unwrap();
    // normal rollback of text
    println!("Deserialized: {:?}", deserialized);
}
