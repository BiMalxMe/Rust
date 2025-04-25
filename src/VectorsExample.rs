fn main(){
    //initalizing the vector
    let mut vec= Vec::new();
    //pushing data in to the vector
    vec.push(1);
    vec.push(10);
    vec.push(2);
    vec.push(6);
    vec.push(44);
    vec.push(109);
    vec.push(15);
    vec.push(13);
    //getting the returned value
    let data:Option<Vec<u32>> = get_even(vec);
    match data {
        Some(num) => {
        print!("Even numbers are ");
        //Iterating through each of the vec datas
        for i in num{
        print!("{},", i);
    }},
        None => println!("No data Found")
}
}
// it returns the vec of u32 size elements
fn get_even(vec: Vec<u32>) -> Option<Vec<u32>> {
    //new vector to store the even
    let mut val  = Vec::new();
    for i in vec {
        if i % 2 == 0 {
            val.push(i);
        }
    }
    //return the whole array enclosing in the option enum
    return Some(val);
}
