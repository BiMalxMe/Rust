fn main() {
    function_one();
}

fn function_one() {
    let str: String = String::from("Bimal Chalise");

    // just passing the reference only ownership is not transferrred
    something(&str);

    // Use str after borrowing
    println!("Inside function_one: {}", str);
}

//takes refrence of string but is not a owner
fn something(word: &String) {
    println!("Hello {}", word);
}
//                                               ||
//                                               ||
//                                               ||
//                                               ||
//                                               ||
//                                               ||
//                                               \/

//If there was not passing of reference
// fn main() {
//     function_one();
// }

// fn function_one() {
//     let str: String = String::from("Bimal Chalise"); //str is owner

//     // just passing the value instead of refernce and ownership is transferrred
//     something(str); //@pass ownership to the word and removes ownership from str

//     // Use str after tranfer of owner
//     println!("Inside function_one: {}", str);//str is no longer any owner of value
    
// }

// //takes value of string but is  a owner
// fn something(word: String) {//get ownership
//     println!("Hello {}", word);//after excevution there will benoowner
// }