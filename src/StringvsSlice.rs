

//using string
fn main1(){
    let name = String::from("Bimal Chalise");
    let passingfunction = find_first_word(&name);
    println!("The First word is {}",passingfunction);
}
//in this new memory allocation is done
//Slower (extra allocation, copying characters)
fn find_first_word(name : &String) -> String {
    let mut newstring = String::new();
    for (_,letter) in name.chars().enumerate(){
        if letter == ' ' {
            break;
        }
        newstring.push(letter);

    }
    newstring
}

//using string Slice
fn main2(){
    let name = String::from("Bimal Chalise");
    let returnedvalue = find_first_word_(&name);
    println!("The firstname is {}",returnedvalue)
}
//In this no new memory allocation is done
// Faster (only slicing; no copying)
//Can only use while original data exists

fn find_first_word_(name : &String) -> &str{
    let mut index = 0;
    for (_,i) in name.chars().enumerate(){
        if i == ' '{
            break;
        }
        index += 1;
    }
    &name[0..index]
}