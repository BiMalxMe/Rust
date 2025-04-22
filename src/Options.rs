// By defaut there is no null value thing in RUST
// so when we have to return the null value we have to use the Option Enum
// which is predefined

fn main() {
    let returnss = find_letter(String::from("Hallocinations"));

    //using the pattern matchers
    match returnss {
        Some(values) => println!("Found 'a' at index: {}", values),
        None => println!("'a' not found in the string"),
    }

    // a
}

pub fn find_letter(s: String) -> Option<i32> {
    for (index, character) in s.chars().enumerate() {
        // let return if there is a in the string
        if character == 'a' {

            //by default the index is usize type(a index default type) 
            //But in the Options enum we are saying that the fn will return the
            //i32 so we have to manually chnage form usize to i32
            return Some(index as i32);
        }
    }
    //return none 
    None
}
