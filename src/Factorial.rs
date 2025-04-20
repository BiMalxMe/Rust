use std::io;

fn main() {
    let mut input = String::new(); 
    //inout should be taken in  a string as there is not 
    //any direct way of taking input as number in rust

    println!("Enter a number:");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    //parsing to u32 as it dont need any kind of negative integer
    let number: u32 = input.trim().parse().expect("Please enter a valid number"); 
    let value : u32 = factorial(number);
    print!("The factorial of {} is {}",number,value)
    //making a function with a return value to make it more complicated



}
fn factorial(num : u32) -> u32{
    let mut fact = 1;
    for i in 1..=num {
        fact *= i;
    }
    return fact;
}