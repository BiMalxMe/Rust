
//     Write a function that takes a vector of tuples(each tuple)
//     containing a key and a value) and returns a Hashmap where 
//     the keys are the unique keys from the input tuples and 
//     the values are vectors of all corresponding values associated
//      with each key


use std::collections::HashMap;

//taking vectors as input and returning the hashmap
pub fn generatehashmaps(x: Vec<(String, u32)>) -> HashMap<String, u32> {
    let mut map = HashMap::new();
    for (key, value) in x {
        map.insert(key, value);
    }
    map
}

fn main(){
    //hardcoding the keys and values 
    let pairs :Vec<(String ,u32)> = vec![
        (String::from("Bimal"),20),
        (String::from("Kristi"),15)
    ];
    //function call with the vector as a input
    let returnedvals = generatehashmaps(pairs);

    //hashmap is like
    //{"Bimal": 20, "Kristi": 15}
    //so useing loop to iterate FOR EACH KEY:VALUE
    for(key,value) in returnedvals{
        println!("{} : {:?}",key,value);
    }

}