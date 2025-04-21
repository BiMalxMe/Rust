
// this program is not fully working there are error and so other which can be later 
// on resolved using enums and all

struct  User{
    first_name : String,
    last_name : String, //can set optional 
    age : u8
}
fn main() {

    // UserDetails---wrong ,user_details --right, Should be in the snake case
    let user_details = User {
        first_name :String::from("Bimal"),
        last_name :String::from("Chalise"),
        age :4
    };
        println!("Hello {}, Your age is {}",user_details.first_name,user_details.age)
}