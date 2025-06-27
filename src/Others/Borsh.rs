use std::vec;

use borsh::{ BorshDeserialize, BorshSerialize};
use borsh_derive::{BorshDeserialize, BorshSerialize};

//used for seializing and deserializing
#[derive(BorshDeserialize,BorshSerialize,Debug)]
struct User{
    name : String,
    email : String,
    age : u32
}

fn main(){
    let data = User{
        name : "Bimal".into(),
        email : "bimal@gmail.com".into(),
        age : 22
    };

    let mut vectr = Vec::new();

     data.serialize(&mut vectr).unwrap();
    println!("{:?}",vectr);

     //conveting the vec byte data into the original form

     let desealized = User::try_from_slice(&vectr).unwrap();
     println!("{:?}",desealized)
}
//no impls