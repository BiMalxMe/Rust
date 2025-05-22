// why use crate::?
//  if the mod is nested deeply
// and if the name conflicts with the external cratef

//example 1
use xyz::mymodeule; //external module

    // my local module
mod my_module { 
    pub fn do_work() { println!("Local work"); }
}

fn main1() {
    my_module::do_work(); // Which one? Compiler error!
    //so using the crate to specify that it is from the current create
    //doing 
    crate::my_module::do_work();
}

//example 2

mod outer {
    mod inner {
        fn foo() {
            // Without `crate::` (confusing)
            super::super::root_fn(); // Where does this come from?

            // With `crate::` (clear)
            crate::root_fn(); //  Obvious it's from the root.
        }
    }
}

fn root_fn() {}