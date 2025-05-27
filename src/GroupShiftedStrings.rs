use std::collections::HashMap;

fn group_shifted_strings(words: Vec<&str>) -> Vec<Vec<String>> {
    let mut groups: HashMap<String, Vec<String>> = HashMap::new();

    for word in words {
        // Generate a key based on the shifting pattern
        let key = get_shift_key(word);
        groups.entry(key).or_default().push(word.to_string());
    }

    groups.into_values().collect()
}

fn get_shift_key(word: &str) -> String {
    if word.is_empty() {
        return "".to_string();
    }
    let bytes = word.as_bytes();
    let mut key = String::new();

    for i in 1..bytes.len() {
        // Calculate the circular difference modulo 26
        // if az then z is 25 letter ahead of the a
        let diff = (26 + bytes[i] as i32 - bytes[i - 1] as i32) % 26;
        key.push_str(&diff.to_string());
        key.push(',');
    }

    key
}

fn main() {
    let words = vec!["abc", "bcd", "xyz", "az", "ba","mln"];
    let grouped = group_shifted_strings(words);

    for group in grouped {
        println!("{:?}", group);
    }
}
