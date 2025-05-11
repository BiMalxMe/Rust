// for mathematical operations
use eval::eval;
use rand::{Rng, seq::SliceRandom};
use std::time::Instant;
use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    //intialziing the number generator
    let mut rng = rand::thread_rng();
    let expressions = ["+", "-", "*", "/"];

    let mut prefix: [u8; 5] = [0; 5];
    let mut suffix: [u8; 5] = [0; 5];

    for i in 0..5 {
        prefix[i] = rng.gen_range(0..=10);
        suffix[i] = rng.gen_range(0..=10);
    }
    let startingtime = Instant::now();
    let mut count = 0;

    for i in 0..5 {
        let operation = expressions.choose(&mut rng).unwrap();
        let mathquestion = format!("{} {} {} = ", prefix[i], operation, suffix[i]);
        //Converting to the integer as float is little hard to digest
        let result = eval(&mathquestion).unwrap().as_f64().unwrap() as u32;
        //print math question in strring format :)
        print!("{} ", mathquestion); 
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut input)
            .expect("Unable to take the input");
        let finalinput: u32 = input.trim().parse().expect("Error Occured while parsing");
        if finalinput == result {
             count = count + 1;
        }
        //clear the input
        input.clear(); 
    }
    let endingtime = startingtime.elapsed();
    //{:.2} -> for two digit after the decimal value
    println!("Total time taken is {:.2?}",endingtime);
    println!("Scored {}/5",count);
}
