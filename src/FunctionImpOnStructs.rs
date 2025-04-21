struct Rect {
    width: u32,
    height: u32,
}

impl Rect {
    //implemnting React on Functions
    fn area(&self) -> u32 {
        self.width * self.height
    }
    //implicit returns
    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }
    //this is the static functions as the check function is not acessing the self
    fn check() {
        println!("Hello From the Check static function");
    }
}
fn main() {
    let rect = Rect {
        width: 10,
        height: 10,
    };
     // Give the area function the data of rect structs
    println!("Area: {}", rect.area());
    // Give the perimeter function the data of rect structs
    println!("Perimeter: {}", rect.perimeter());
    //Way of acessing the static functions
    Rect::check();
}
