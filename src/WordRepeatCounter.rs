use std::collections::HashMap;
use std::io;

fn word_frequency(sentence: &str) -> HashMap<String, usize> {
    
    //initaialize a new hashmao
    let mut freq_map = HashMap::new();

    //iterate over the hashmap
    for word in sentence.split_whitespace() {
        // Convert &str to String
        let word = word.to_string(); 
        //if entry is available insert +1 else create and put default value 0
        *freq_map.entry(word).or_insert(0) += 1;
    }

    //return hashmap
    freq_map
}

fn main() {
    let mut sentence = String::new();
    //takes input
    io::stdin()
    .read_line(&mut sentence)
    .expect("Error while taking input");
    
    //trim and to lowercase cant be used in same because they retuen differnet types
    let lowercased = sentence.to_lowercase();
    let finalinputeddata = lowercased.trim();

    let finalhash = word_frequency(finalinputeddata);

    for (word, count) in finalhash {
        println!("{}: {}", word, count);
    }
}
