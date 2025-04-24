
// This works fine as there is no mutable reference 

fn main1(){
    let str = String::from("Bimal Chalise");
    let s1 = &str;
    let s2 =  &str;
    let s3 =  &str;
     println!("{},{},{}",s1,s2,s3);
}


//This code explains that id there is any one mutable reference then other refernce can be proceeded

fn main2(){
    let mut str = String::from("Bimal Chalise");
    let s1 = &mut str;
    let s2 =  &str;
    let s3 =  &str;
     println!("{},{},{}",s1,s2,s3);
}//error[E0502]: cannot borrow `str` as immutable because it is also borrowed as mutable

//problem is that Rust doesn’t allow having both a mutable reference and an immutable reference 
//to the same data at the same time. This is to prevent possible errors or unexpected behavior.