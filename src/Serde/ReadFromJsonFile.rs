use serde::{Deserialize, Serialize}; 
use tokio::fs::read;             

#[derive(Debug, Deserialize, Serialize)] // Derive Debug, Deserialize, and Serialize traits
struct Student {
    roll_no: u8,      // Roll number (unsigned 8-bit integer)
    name: String,    // Name (String)
    marks: u8,         // Age (unsigned 8-bit integer)
}

#[tokio::main] // use tokio's runtime for async operations
async fn main() {
    let filename = "student.json"; // store filename in a variable for reusability

    // Read the file asynchronously
    let file_content = match read(filename).await {
        Ok(content) => content, // If successful, store the file content
        Err(e) => {
            eprintln!("Error reading {}: {}", filename, e); // Print error to standard error
            return; // Exit the program on error
        }
    };

    // deserialize the JSON data into a Vec<Student>
    let students: Vec<Student> = match serde_json::from_slice(&file_content) {
        Ok(data) => data, // If successful, store the vector of students
        Err(e) => {
            eprintln!("Error deserializing JSON from {}: {}", filename, e); // Print error to standard error
            return; // Exit the program on error
        }
    };

    // print a success message with the number of students
    println!("Successfully read and deserialized {} students:", students.len());

    // tterate over the first 5 students and print their details
    for student in students.iter().take(5) {
        println!("{:?}", student); // Use Debug trait to print the student struct
    }
}
