use std::collections::HashMap;

fn main() {
    //defining some vector datas
    let data = vec![123, 234, 345, 124, 135, 246, 120, 456, 111, 222, 987];
    //get the groups
    let grouped = groupby_shifts(data);

    //print all the inner vector seperateyl to seperate according to the shift
    for group in grouped {
        println!("{:?}", group);
    }
}

fn groupby_shifts(nums: Vec<u32>) -> Vec<Vec<u32>> {
    //create a new hashmap collection
    let mut alldatas: HashMap<String, Vec<u32>> = HashMap::new();

    //iterate over the num in the first initailized vector
    for num in nums {
        //get the shif key
        let key = get_shift_key(num);

        //alldatas.entry() -> search from the key value pair with the key "key"
        //and once it finds pushes the num into the values
        //or_default() -> gives acess to value of key if value exists else i will create a new vec value and gives mutbale reference
        alldatas.entry(key).or_default().push(num);
    }

    //just get the values and collect in the return type
    alldatas.into_values().collect()
}

fn get_shift_key(num: u32) -> String {
    let digits: Vec<u32> = num
        .to_string()
        .chars()
        // to difit accepts a argument radix where we give base
        //like 10-decimal 2-binary 16-hex and so other
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    //if the num is single digited then ther is no way to shift
    if digits.len() <= 1 {
        return "0".to_string(); // Single digit numbers or invalid
    }

    let mut key = String::new();
    for i in 1..digits.len() {
        let diff = digits[i] as i32 - digits[i - 1] as i32;
        key.push_str(&diff.to_string());
        key.push(',');
    }

    key
}
