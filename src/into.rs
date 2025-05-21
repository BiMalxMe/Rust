use std::path::PathBuf;

//into works on one condition that there should not be a data loss
 fn main(){
    // this is a  &str
    let data = "bimal chalise";

    // change the f32 to f64 using the into fn
    let num :f32 = 2.9;
    let intval : f64 = num.into();

    println!("The integer value is {}",intval);
    // into() changes the data into the string format as type alloted
    let finalstringdata :String = data.into();
    println!("The string data is {}",finalstringdata);

    //send the same str data into the arg such that it will be changed according to the type of 
    // parameter type annotation
    getthedataintheformofint(data.into());
    print!("{}",data)
 }
 fn getthedataintheformofint(a : PathBuf){
    //changes the str into the pathbuf
    println!("The data is {:?}",a)
 }

 //--------OUTPUT

//  The integer value is 2.9000000953674316
// The string data is bimal chalise
// The data is "bimal chalise"
// bimal chalise% 