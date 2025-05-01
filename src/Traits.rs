
//Used to impleemnt the shared behaviour with certain format
trait Greet {
    fn greet(&self);
}

//defining a empyty structs
struct Dog;
struct Robot;
// implementing the Greet trait in the structs 
impl Greet for Dog {
    //must use greet function as it is specified by the trait
    fn greet(&self) {
        println!("Woof!");
    }
}
impl Greet for Robot {
    fn greet(&self) {
        println!("Beep bop.");
    }
}

// generic function using trait bound and thing is a struct
fn greet_anyone<T: Greet>( thing : T) {
    thing.greet();
}

fn main() {
    //making a obj of structs 
    let d = Dog;
    let r = Robot;

    //passing the arguments
    greet_anyone(d);
    greet_anyone(r);
}
