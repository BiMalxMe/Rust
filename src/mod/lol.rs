pub fn bimalchalise(str: &str) {
    println!("My name is {}", str);
}

pub mod lolkochora {
    pub mod sathi {  
        pub static DATA: i32 = 234; 
        pub fn print_data() {  // Added a function to print the data
            println!("This is the sathi function, and data is: {}", DATA);
        }
    }
}
