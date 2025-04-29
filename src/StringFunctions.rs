fn main() {
    // Creating strings
    let mut s = String::from("hello");
    let s_literal = "world";
    let s2 = s_literal.to_string();

    // Appending
    s.push(' ');             // Add a space
    s.push_str(&s2);         // Append "world"

    println!("Appended string {}", s);

    // Insertion
    s.insert(0, '*');        // Insert '*' at the beginning
    println!("After insert {}", s);

    // Remove last character
    s.pop();
    println!("After pop: {}", s);

    // Length and emptiness
    println!("Length in bytes {}", s.len());
    println!("Is empty {}", s.is_empty()); //true and false

    // Search
    println!("Contains 'hello' {}", s.contains("hello"));//true or false
    println!("Starts with '*hello' {}", s.starts_with("*hello"));

    // Replace
    let replaced = s.replace("world", "Rust");
    println!("After replace: {}", replaced);

    // Split
    for word in s.split(' ') {
        println!("Split word {}", word);
    }

    // Trimming
    let trimmed = "  hello trimmed  ".trim();
    println!("Trimmed: '{}'", trimmed);

    // Case conversion
    println!("Uppercase: {}", s.to_uppercase());
    println!("Lowercase: {}", s.to_lowercase());

    // Parse
    let num_str = "42";
    let num: i32 = num_str.parse().unwrap();
    println!("Parsed number + 1 {}", num + 1);
}


//answers

// Appended string hello world
// After insert *hello world
// After pop: *hello worl
// Length in bytes 11
// Is empty false
// Contains 'hello' true
// Starts with '*hello' true
// After replace: *hello worl
// Split word *hello
// Split word worl
// Trimmed: 'hello trimmed'
// Uppercase: *HELLO WORL
// Lowercase: *hello worl
// Parsed number + 1 43