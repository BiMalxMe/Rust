 
 // Creating a multiplicationn table
//   from 1-10 and printing in ten differenct files

use std::fs::File;
use std::io::{self, Read, Write};

//As we are working with files so using io for result 
fn main() -> io::Result<()>{
    for i in 1..10{
    printtable(i)?;
}
//now reading the files
    readfile()?;
Ok(())
}
fn printtable(x : u8) -> io::Result<()>{ 
    // Creates "file_1.txt"
    let filename = format!("file_{}.txt",x); 
    println!("{}", filename);
        let mut file = File::create(filename)?;
    for i in 1..11{
        let formattedstring = format!("{} x {} : {}\n",x,i,x*i);
       //We need to convert to the byte as the fn expects byte not string
        file.write_all(formattedstring.as_bytes())?;
    }
    Ok(())
}
//reading file that are created
fn readfile() -> io::Result<()>{
    for i in 1..11{
        // ? -> If error ocurs handles and panics
    let filename = format!("file_{}.txt",i); 

    let mut content = String::new();
    let mut file = File::open(filename)?;
    file.read_to_string(&mut content)?;
    println!("{}",content)
}
Ok(())
}