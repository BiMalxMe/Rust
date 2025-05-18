
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, Debug)]
struct User {
    username: String, 
    age: u8,          
}

fn main() {
    // Create a new instance of the User struct
    let user = User {
        username: "DrBimal".to_string(), 
        age: 28,                         
    };


    //convert the struct into a u8 integer such that it points the character
    let bytes = serde_json::to_vec(&user).unwrap();       // Serialize
    //print in the form of vector
    println!("{:?}",bytes);
}


