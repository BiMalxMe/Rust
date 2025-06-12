fn main() {
    // No escaping for quotes (escaping means insetad of "" using the \"\")
    let raw_str = r#"Hello, "Rust"!\nLiteral \n and \"."#;
    println!("{}", raw_str);

    // Windows path without escaping
    //DOESNOT ALLOW QUOTES
    let raw_path = r"C:\Users\file.txt";
    println!("{}", raw_path);

    // Multi-line string ALLOW QUOTES
    let multiline = r#"
        Line 1
        Line 2 "with quotes"
        Line 3 \n (not a newline)
    "#;
    println!("{}", multiline);

    // JSON data (no escaping needed)
    let json = r#"{ "name": "Alice", "age": 30 }"#;
    println!("{}", json);
}
//              OUTPUT
//              -------
// Hello, "Rust"!\nLiteral \n and \".

// C:\Users\file.txt

//         Line 1
//         Line 2 "with quotes"
//         Line 3 \n (not a newline)
    
// { "name": "Alice", "age": 30 }


//   MOStly used bcozz
// JSON example - much cleaner with raw strings:
// let json_normal = "{ \"name\": \"Alice\", \"age\": 30 }";  // Messy
// let json_raw = r#"{ "name": "Alice", "age": 30 }"#;       // Clean