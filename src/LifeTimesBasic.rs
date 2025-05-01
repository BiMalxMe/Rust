//program to demonstare the lifetime in rust

fn findthelargest<'a>(word1: &'a str, word2: &'a str) -> &'a str {
    if word1.len() > word2.len() {
        word1
    } else {
        word2
    }
}
fn main() {
    let ans;
    let first_word = String::from("Bimal Chalise");

    let second_word = String::from("Elon Musk");
    // if not used lifetime gives ERROR
    // |                          ----         ----     ^ expected named lifetime parameter
    ans = findthelargest(&first_word, &second_word);
    //makeing the ans lifetime as long as the least among the word1 or word2
    println!("The largest word is {}", ans)
}
