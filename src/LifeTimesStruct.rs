//have the same lifetime because the scope is equall
struct User<'a>{
    //if theere is reference inside the struct then therer lifetimes should be used compulsary
    firstname : &'a str,
    lastname : &'a str
}

fn main(){

    let fname = String::from("Bimal");
    
    let lname = String::from("Chalise");
    //referencing to the fname and lname
    let user = User {
        firstname : & fname,
        lastname : & lname
    };
    println!("The full name of the person is {} {}",user.firstname,user.lastname)
}