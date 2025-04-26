
// Write a Rust program that takes user input for a list of numbers, filters the odd numbers,
// filters the odd numbers, doubles them, and stores the results in a new vector.
// Display both the original and new vectors.

use std::io;

fn main(){
    let mut vectrr: Vec<u32> = Vec::new();
    println!("Enter how many number you want to input");
    
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    
    let total: u32 = input.trim().parse().expect("An error occurred while parsing");

    //upto total
    for i in 1..=total {
        println!("Enter the position {} number ", i);
        let mut elements = String::new();
        io::stdin()
            .read_line(&mut elements)
            .expect("Failed to read line");

        let final_elements: u32 = elements.trim().parse().expect("An error occurred while parsing");
        vectrr.push(final_elements);
    }

    let returned_val: Vec<u32> = odd_filter(vectrr.clone());

    println!("The original vector is {:?}", vectrr);
    println!("The double of the odd elements is {:?}", returned_val);
}

fn odd_filter(x: Vec<u32>) -> Vec<u32> {
    let mut oddvectrr: Vec<u32> = Vec::new();
    let finalvariable = x.iter()
        .filter(|&num| num % 2 != 0)
        .map(|odd| odd * odd);

    for i in finalvariable {
        oddvectrr.push(i);
    }

    oddvectrr
}
