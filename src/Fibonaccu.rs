use std::io;

fn main() {
    let mut number = String::new();

    println!("Enter any number:");

    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line");
    //in this place the input value taken is in string type

    //so parsing into string such that there will be no any type confusions

    let parsed_number : u32= number.trim().parse().expect("Failed to parse the input value");
    fibo(parsed_number)

}

pub fn fibo(x:u32){
    let mut first: i32 = 0;
    let mut second: i32 =1;
    
    print!("{},{},",first,second);
    for _ in 1..x{
        let temp =first + second;
        first = second;
        second = temp;
        print!("{},",temp);
    }
}
