// Using the math functions in Rust

fn main() {
    let number: f64 = 21.5;

    //Rounds up the number
    println!("{} after ceil is : {}", number, number.ceil());

    //Rounds down the number
    println!("{} after floor is : {}", number, number.floor());

    //sqrt
    println!("{} after sqrt is : {}", number, number.sqrt());

    //fract -> gives the decimal value
    println!("{} after fraction is : {}", number, number.fract());

    //rounds up to the nearest integer
    println!(
        "{} after rounding up to nearest is {}",
        number,
        number.round()
    );

    //trunc -- gives the integer vefore the decimal value
    println!("{} after tuncating is : {}", number, number.trunc());

    //powi -- gives the power in integer val
    println!(
        "{} after squaring with power 2 is : {}",
        number,
        number.powi(2)
    );

    //powf -- gives the power in flaoting val
    println!(
        "{} after squaring with power 2 is : {}",
        number,
        number.powf(2.5)
    );

    //printing the maximum and the minimum value
    println!(
        "For the values 10 and 11:\n- The max is {}\n- The min is {}",
        10.max(11),
        10.min(11)
    );

    
}
