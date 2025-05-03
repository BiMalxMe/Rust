use std::fmt::Display;

//as two refs are sent then we need the lifetime
fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    anns: T,
) -> &'a str
where
// as display trait is imlplemented by anns it can print multiple data type prints
    T: Display,
{
    println!("Announcement: {anns}"); // announcing before data result
    //normal calculations
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let str1 = "Bimal";
    let str2 = "Chalise";

    //this time passing the string slice not owned and compiled as it is in binary
    let announcement = "This is the longest string!";
    
    //sedning the args
    let longest = longest_with_an_announcement(str1, str2, announcement);
    println!("The longest string is: {}", longest);
}
