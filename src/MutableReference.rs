

//This code make sure that a fn can modify the string data and make sure that refernce is 
// used as the change will be global without changing of the owner



// In this whole code only str is the only owner i.E ownership is not trnasferred
fn main() {
    function_one();
}

fn function_one() {
    let mut  str: String = String::from("Bimal Chalise");

    // pass the mutable string to the function
    println!("Before Passing To The function_one: {}", str);//Before Passing To The function_one: Bimal Chalise

    something(&mut str);

    // Print the Updated String
    println!("After passing to the function_one: {}", str);//After passing to the function_one: Bimal Chalise is a genius
}

//modifing the Sting Defined Above
fn something(word : &mut String){
    word.push_str(" is a genius");
}