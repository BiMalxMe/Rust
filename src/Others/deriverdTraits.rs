use std::fmt;

struct User {
    username: String,
    email: String,
    age: u32,
}

// Automatically implement Debug using derive
#[derive(Debug)]
struct SecondOne {
    username: String,
    email: String,
    age: u32,
}
// How a end user will see the output
// Manually implement Display as no derive available
impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      // format the waty of print using write! macro
        write!(f, "{} ({}) is {} years old", self.username, self.email, self.age)
    }
}

fn main() {
    let user1 = User {
        username: String::from("Bimal"),
        email: String::from("bimal@gmail.com"),
        age: 28,
    };

    let user2 = SecondOne {
        username: String::from("ramesh"),
        email: String::from("ramesh@gmail.com"),
        age: 35,
    };
 //output for user to see
    println!("{}", user1);
// for a developer to debug
    println!("{:?}", user2);
}

