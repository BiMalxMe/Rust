//traits as a parameter

pub trait Subject{
    fn describe(&self) -> String ;
}
struct Student{
    name : String,
    major : String
}
impl Subject for Student {
    fn describe(&self) -> String {
        return  format!("{} has the major subject {} ",self.name,self.major)
    }
}
struct Teacher{}

pub fn demo_to_see_if_traits_implemented(something : impl Subject){
    println!(" {} ",something.describe())
}

fn main(){
    let student_info = Student{
        name : String::from("Bimal Chalise"),
        major : String::from("Computer")
    };
    //as the teacher struct is not implementing the Subject trait so it cant be taken as a parameter 
    // in the impl Subject function -> demo_to_see_if_traits_implemented
    let otherstruct = Teacher{};
    println!("{}", student_info.describe());
    //It cant be passed 
    demo_to_see_if_traits_implemented(otherstruct);
    //this can be implemented as it used the subject trait
    demo_to_see_if_traits_implemented(student_info);
    
}