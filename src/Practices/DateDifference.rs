use chrono::{self, NaiveDate};
use std::io::{self, Write};

fn main() {
    let mut date1 = String::new();
    let mut date2 = String::new();
    println!("Enter Two dates in YYYYMMDD format");
    print!("Enter the First Date : ");
    //implicitely used in the print
    io::stdout().flush().unwrap(); // flush to show prompt
    io::stdin()
        .read_line(&mut date1)
        .expect("An error occurred while parsing");
    print!("Enter the Second Date : ");
    io::stdout().flush().unwrap(); // flush to show prompt

    io::stdin()
        .read_line(&mut date2)
        .expect("An error occurred while parsing");
    let finaldate1 = NaiveDate::parse_from_str(&date1.trim(), "%Y-%m-%d").unwrap();
    let finaldate2 = NaiveDate::parse_from_str(&date2.trim(), "%Y-%m-%d").unwrap();

    //user is dumb , might give smaller and grater or vice versa so neg val may
    //arise so doing checks
    let timebetweenthem ;
    if finaldate1>finaldate2 {
     timebetweenthem = finaldate1 - finaldate2;
}else if finaldate2>finaldate1 {
    timebetweenthem = finaldate2-finaldate1;
}else{
    println!("The given dates are equal");
    return;
}
    // the days difference is given
    println!(
        "There is difference of {} days ",
        timebetweenthem.num_days()
    )
}
