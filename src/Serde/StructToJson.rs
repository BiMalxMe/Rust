
use serde::{Serialize, Deserialize};
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


    // preety means the actual json format
    let pretty_json = serde_json::to_string_pretty(&user).unwrap();
    println!("Pretty JSON:\n{}", pretty_json);
}