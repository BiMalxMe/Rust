//Filter map foes the work of filter and map at once
fn main() {
    let data = vec!["1", "two", "3", "four", "5"];
    let numbers: Vec<u32> = data
        .iter()
        //True conditions remains and other gets deleted
        //only data with sucessfull parsing will be remained
        .filter_map(|s: &&str| s.parse::<u32>().ok())
        .collect();
    println!("Original data: {:?}", data);
    //successful ones
    println!("Parsed numbers: {:?}", numbers);
}
