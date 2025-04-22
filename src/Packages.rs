

// package managemnt

// => Process of adding the external crates/packages into the local system
// use can add crate by using : cargo add _____(crateName)


//for example

// Utc --> universal time
// local --> my local time


use chrono::{Utc,Local};

fn main(){

    // as the now is the static function we ate usign the ::
    let current_time_at_utc = Utc::now();
    println!("The Current time is {}",current_time_at_utc);

    let current_time_at_local = Local::now();
    println!("The Current time is {}",current_time_at_local);


}