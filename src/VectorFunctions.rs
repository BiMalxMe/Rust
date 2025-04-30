fn main() {
    //  Creation
    let _v: Vec<i32> = Vec::new();
    let mut v2 = vec![1, 2, 3];
    let _v3 = vec![0; 5]; // [0, 0, 0, 0, 0]

    //  Adding Elements
    v2.push(4);
    //adds the more numbers
    v2.extend_from_slice(&[5, 6]);

    //  Accessing Elements
    println!("First: {}", v2[0]);              // Index (panics if out of bounds)
    println!("Second: {:?}", v2.get(1));       // Safe access

    //  Removing Elements
    v2.pop();      
    //remove at the index 2                            // Removes last
    v2.remove(2);                              // Removes at index 2
    let mut v4 = vec![10, 20, 30];
    v4.clear();                                // Removes all elements

    //  Inserting Elements
    //at index 1 element 99
    v2.insert(1, 99);                          // Insert at index 1

    //  Querying
    println!("Length: {}", v2.len());
    println!("Is Empty: {}", v2.is_empty());
    println!("Contains 3? {}", v2.contains(&3));
    match v2.iter().position(|&x| x == 3) {
        Some(index) => println!("Found at index: {}", index),
        None => println!("Not found"),
    }

    //  Iteration & Transformation
    for val in &v2 {
        println!("Value: {}", val);
    }
    let doubled: Vec<i32> = v2.iter().map(|x| x * 2).collect();
    println!("Doubled: {:?}", doubled);

    //  Sorting & Reversing
    let mut nums = vec![5, 3, 1, 4, 2];
    nums.sort();
    println!("Sorted: {:?}", nums);
    nums.reverse();
    println!("Reversed: {:?}", nums);

    //  Other Utilities
    let mut dupes = vec![1, 1, 2, 2, 2, 3];
    // Removes consecutive repeated elements in the vector 
    dupes.dedup();
    println!("Deduped: {:?}", dupes);

    let mut trunc = vec![1, 2, 3, 4, 5];
    trunc.truncate(3); // Keeps first 3 elements
    println!("Truncated: {:?}", trunc);

    let mut split_vec = vec![1, 2, 3, 4, 5];
    //splitted into two vecs
    let second_half = split_vec.split_off(2);
    println!("First half: {:?}", split_vec);
    println!("Second half: {:?}", second_half);
}
