 
 pub fn main(){
    let x = 1;
    let y = 100;
    let sum = do_sum(x,y);
    println!("Hey Bimal, the sum is {}",sum);
 }
 fn do_sum(a:i32,b:i32) -> i32{
    return a+b;
 }