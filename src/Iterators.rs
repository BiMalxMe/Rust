
//______________________Understanding the Iterators____________________

//under the hood uses the --------------------->into_iter()
fn main(){
    let vecs = vec![1,2,3,4];
    //ownership is transferred to i
     for i in vecss{
        println!("Got {} ",i);
     };

     //It cant print as the valueis already moved
     println!("{:?}",vecs);
}

//-----------------------iter()---------------------------------------------
// but using iter() function we can pass the referecnce only
fn main1(){
    let vecs = vec![1,2,3,4];
    //ownership is not transferrred as reference is passed only to i
     for i in vecs.iter(){
        println!("Got {} ",i);
     };

     //It can print as it is owner
     println!("{:?}",vecs);
}

// //-----------------------iter.mut()---------------------------------------------
fn main2(){
    let mut vecs  = vec![1,2,4];
    //iterating over the mutable reference
    let final_vec = vecs.iter_mut();
    for i in final_vec{
        //changin the value as mutable refernce is passed
        //point to the original value
        *i = *i*10;
        println!("The values is {}",i);
        
    }
    println!("The original vector is now changed into {:?}",vecs)
}


//----------------------------------.next()---------------------------------------------
fn main3(){
    let mut vectr = vec![1,2,3,4,5];
    //sending the mutable reference such that it can change the original vals
    let mut  final_vec  = vectr.iter_mut();
    
    //another way 
    while let Some(val) = final_vec.next(){
        *val += 2;
        println!("{} ,",val);
    }
    //still the vectr has ownership
    print!("{:?}",vectr)
}