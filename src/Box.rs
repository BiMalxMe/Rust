
//Making a enum
//without the box, the code can't compile bcoz of Not fixed size
// and changeing the unpredictable to the fixed size
enum List{
    Con(i32,Box<List>),
    Nil
}
// Use the enum declared above
use List::{Con,Nil};

fn main(){
    // Create a list with the recursive function
    // and box is given as the List expects
    let list: List = Con(1, Box::new(Con(2, Box::new(Con(3, Box::new(Nil))))));
   // moving reference such that list can be used further more in next steps
    print_list(&list);
}
// declaring the function print/list 
fn print_list(list : &List){
    match list {
        Con(val,next ) => {
            print!("{} -> ",val);
            print_list(&next);
        }
        Nil => print!("Nil"),
    }
}