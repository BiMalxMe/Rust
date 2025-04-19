fn main() {
    let name1: String = String::from("bimal"); 
    // Creates a new heap-allocated String and assigns ownership to name1

    let name2: String = name1;  
    // Moves ownership from name1 to name2
    // name1 is no longer valid after this point

    // println!("{}", name1); 
    // Error: name1 has been moved and cannot be used

    println!("{}", name2);     
    // Prints "bimal", because name2 now owns the String

    // -----------------------------
    // Rust Ownership Rules:
    // 1. Each value in Rust has a single owner
    // 2. When the owner goes out of scope, the value is dropped
    // 3. Ownership can be transferred (moved), but only one owner at a time
}
