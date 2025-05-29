use std::collections::HashMap;

fn main() {
    //initialize the data vec
    let data = vec![111, 234, 345, 124, 135, 246, 120, 456, 111, 222, 987];
    //get the datas based on different groups
    let grouped = groupby_digit_sum(data);

    //iteratte through the vec
    for group in grouped {
        println!("{:?}", group);
    }
}

fn groupby_digit_sum(nums: Vec<u32>) -> Vec<Vec<u32>> {
    // initalize new hashmap
    let mut alldatas: HashMap<u32, Vec<u32>> = HashMap::new();

    //if num =123 then get the sum each from the get_digit_sum() 
    for num in nums {
        let key = get_digit_sum(num);
        alldatas.entry(key).or_default().push(num);
    }

    //get values only from the hahmap and collect into the vec
    alldatas.into_values().collect()
}

// it does this
// if num = 111
//stringify it = "111"
//chars = "1" "1" "1"
//map and changes to decimal digit= 1   1    1
// sum  = 1+1+1;
//returns num
fn get_digit_sum(num: u32) -> u32 {
    num.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .sum()
}
