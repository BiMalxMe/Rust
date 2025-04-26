
// types of adpters in iterator



// ✅ Iterator Adapters
// Return new iterators.

// Are lazy – they don’t run until consumed.

// Example: .map(), .filter()

// ✅ Consuming Adaptors
// Return a final result, not another iterator.

// Are eager – they run and consume the iterator.

// Example: .sum(), .collect(), .count()



//consuming adapters

fn main1(){
    let vectr = vec![1,2,3,4];

    //once used then can be use as ownership is already taransferred
    let final_vec = vectr.iter();

// retursn final value
    let sumtotal : i32 = final_vec.sum();
    print!("the toatl sum is {}",sumtotal);

    //borrow of moved value: `final_vec`--cant use as it is used previously by sumtotal
    println!("{}",final_vec)
}

//Iterator Adapters
fn main2(){
    let vectr = vec![1,2,3,4,5];
    let final_vec = vectr.iter();

    //returns new iterator
    let egvector = final_vec.map(|c| c*c);
    for i in egvector{
        print!("values are {}, ",i);
    };
}