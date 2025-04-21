enum Shapes{
    Rectangle(f64,f64),//float value of 64bit
    Circle(f64)
}
fn main(){
    let choosenshape1 = Shapes::Circle(7.0);
   let returnedval1 = calculate_area(choosenshape1);
   println!("The area of the ractangle is {}",returnedval1);
    let choosenshape2 = Shapes::Rectangle(7.0,8.0);
   let returnedval2 = calculate_area(choosenshape2);
   println!("The area of the Circle is {}",returnedval2)
}


fn calculate_area(shapeval : Shapes) -> f64{
// match in Rust is like a way to check different possibilities for a value and do 
// something depending on what it is. It's like saying, "If this is 1,
// do this; if it's 2, do that; otherwise, do something else."

    match shapeval{
        Shapes::Rectangle(x,y ) => x*y,
        Shapes::Circle(x)=> 3.14*x*x
    }
}