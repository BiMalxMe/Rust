fn main(){
    let admakadoodh = String::from("TEhi ;)");
    // It the borrowed value
    //age is the parameter received from the prindata fn parenthesis
    let printdata =  |age: i32| {
        println!("The doodh is {} and the age is {}",admakadoodh,age)
    };
    printdata(34);
    // as it is borrowed value it can be used again
    println!("This is what it is {} ",admakadoodh);
}