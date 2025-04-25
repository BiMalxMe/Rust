

// Implementation Of hashmap collections
//Import  this for using hashmap

use std::collections::HashMap;

fn main(){

    //___hashmaps are like the JSON in javascripts and all___


    let mut user: HashMap<String, i32> = HashMap::new();
    //__________________________________insert()_____________________________________________
    user.insert( String ::from("Bimal"), 22);
    user.insert(String::from("kristi"), 17);

    //using the Option enum to get the data and error msg based on the availablilty if the key


    // let find_age: Option<&i32> = user.get("Bimdal");//Sorry the user is not found in the db

    //_________________________________get()__________________________________________
    let find_age: Option<&i32> = user.get("Bimal");//The age off the reuqested individual is 22

    //pattern matching
    match find_age {
        Some(age)=> println!("The age off the reuqested individual is {}",age),
        None => println!("Sorry the user is not found in the db"),
    }
    //_______________________________remove()_______________________________________________
    let removed = user.remove("Bimal");
    match removed {
        Some(val) => println!("Removed Bimal with age: {}", val),
        None => println!("Key not found"),
    }
    println!("After deleting the Bimal {:?}",user);
    //_______________________________clear()___________________________________________________
    user.clear();
    println!("After complete clearning {:?}",user)
}