fn main(){
    let is_male = false ;
    let is_above_18 = true;

    if is_male{
        println!("You are male");
    }else{
        println!("You are not a male");
    }

    if is_male && is_above_18{
        println!("You are a legal male");
    }
}