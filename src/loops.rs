

pub fn main(){
    let str = String::from("Bimal Chalise");
    println!("First Name {}",get_firstname(str));
}

pub fn get_firstname(str:String) -> String{
    let mut first_name = String::from("");
    for c in str.chars(){
        if  c == ' ' {
            break
        }
        first_name.push(c);
    }
    return first_name;
}