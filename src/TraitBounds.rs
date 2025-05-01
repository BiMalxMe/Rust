//First trait
pub trait Say1 {
    fn sayone(&self) {
        let s1 = String::from("Hi there From Say1");
        println!("{}", s1);
    }
}
//Seconf trait
pub trait Say2 {
    fn saytwo(&self) {
        let s2 = String::from("Hi there From Say2");
        println!("{}", s2);
    }
}
///empty struct user
struct User {
}
//empty struct Fix for checking trait bounds
struct Fix{}
//implementing traits in the user struct
impl Say1 for User {}
impl Say2 for User {}

// Use trait bounds to accept any type that implements Say1 + Say2
fn checking_trait_bounds<T: Say1 + Say2>(something: T) {
    something.sayone(); // These print internally, so no need for println! again
    something.saytwo();
}

fn main() {
    let user = User {
    };
let fix = Fix{};
    checking_trait_bounds(user);
    //cannot pass the fix as it doesnot implements the Say1 and Say2 but the checking_trait_bouds
    //function expects you to implement the both of the impls
    checking_trait_bounds(fix);// X

}
