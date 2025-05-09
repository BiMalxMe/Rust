use std::time::Instant;

use chrono::{Local, NaiveDate, NaiveTime};

fn main(){
 let val = Local::now();
 println!("The current local date is {}",val.format("%Y-%m-%d"));
 println!("The current time is {}",val.time().format("%H:%M"));

 //just print the pure date
 println!("The pure date is {}",val.date_naive());

 //parsing the date from the specific string
 let date_str = "20250509";
 //giving the format of the input string
    let parseddate = NaiveDate::parse_from_str(date_str,"%Y%m%d");
    match parseddate {
        Ok(parsed_date) => println!("Successfully parsed date: {}", parsed_date),
        Err(e) => println!("Error parsing date: {}", e),
    }


    let time_str = "045230";
    let parsedtime = NaiveTime::parse_from_str(time_str, "%H%M%S").unwrap();
    println!("The parsed time is {}",parsedtime);

    // time elapsed thing
    let timenow = Instant::now();
    let mut sum = 0;
    for _ in 0..1000{
        sum = sum + 1;
    };
    let totaltimetaken = timenow.elapsed();
    println!("The total time from 29 to 33 line is {:?}",totaltimetaken);
}