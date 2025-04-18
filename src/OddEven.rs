fn main() {
    let number = 5;

    let remainder = number % 2;

    if remainder == 0 {
        println!("{} is even", number);
    } else {
        println!("{} is odd", number);
    }
}
