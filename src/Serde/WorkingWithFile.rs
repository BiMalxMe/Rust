use std::fs;
use serde::{Serialize, Deserialize};
use serde_json; 

#[derive(Serialize, Deserialize, Debug)]
struct User {
    username: String,
    age: u8,
}

fn main() {
    // Create an instance of User
    let user = User {
        username: "DrBimal".to_string(),
        age: 28,
    };

    // Serialize user to JSON string
    let json = serde_json::to_string(&user).unwrap();

    // Save JSON string to file "user.json"
    fs::write("user.json", &json).expect("Unable to write file");

    // Read JSON string back from the file
    let json_from_file = fs::read_to_string("user.json").expect("Unable to read file");

    // Deserialize JSON string back into a User struct
    let user_from_file: User = serde_json::from_str(&json_from_file).unwrap();

    // Print the deserialized struct
    println!("Deserialized user: {:?}", user_from_file);
}
