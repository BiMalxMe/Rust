
//create the pub global module
mod neighborhood {
    // inner public module fn street
    pub fn street() {
        println!("Main street");
    }
    //second parent module
    pub mod house {         
        pub mod bedroom {   
            pub fn find_street() {
                //acess the two parent previous street module fn
                super::super::street(); 

                // dont have to write self comulsarily
                // dont to say that this acess the fn from the current module
                self::street();
            }
            // create a function street in the same module
            pub fn street(){
                println!("Not a main street")
            }
        }
    }
}
fn main() {
    neighborhood::house::bedroom::find_street();  
}