// Function that returns the longest among three string slices

//manually define the lfetime of result as the least among the parameters
fn longest<'a>(x: &'a str, y: &'a str, z: &'a str) -> &'a str {
    if x.len() > y.len() && x.len() > z.len() {
        x
    } else if y.len() > z.len() {
        y
    } else {
        z
    }
}

fn main() {
    let a = String::from("Sapkota");
    let b = String::from("neplai");
    let c = String::from("Bimal Gates");

    //result lifetime is set
    let result = longest(&a, &b, &c);
    println!("The longest name is: {}", result);
}
