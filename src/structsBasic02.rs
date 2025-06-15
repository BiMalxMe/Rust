// 1. define a struct named product
struct Product {
    name: String,
    price: f64,
}

// implement methods and associated functions for the product struct
impl Product {
    // an "associated function" (like a constructor) to create a new product
    // it doesn't take `&self`, so it's called on the struct itself (e.g., `product::new(...)`)
    fn new(name: &str, price: f64) -> Product {
        Product {
            name: name.to_string(), // convert &str to string for the struct's owned field
            price, // shorthand for `price: price`
        }
    }

    // a "method" to display the product's details
    // `&self` means it borrows the product instance immutably (read-only access)
    fn display_details(&self) {
        println!("Product: {}", self.name);
        println!("Price: ${:.2}", self.price); // {:.2} formats to 2 decimal places
        println!("--------------------");
    }

    // another method: check if the product is expensive (e.g., price > 50.0)
    // also takes &self for read-only access
    fn is_expensive(&self) -> bool {
        self.price > 50.0
    }
}

fn main() {
    // create instances of the product struct using the `new` associated function
    let laptop = Product::new("Laptop", 1200.50);
    let mouse = Product::new("Wireless Mouse", 25.99);
    let keyboard = Product::new("Mechanical Keyboard", 85.00);

    println!("--- product catalog ---");

    // call the display_details method on each product instance
    laptop.display_details();
    mouse.display_details();
    keyboard.display_details();

    // use the is_expensive method
    println!("is {} expensive? {}", laptop.name, laptop.is_expensive());
    println!("is {} expensive? {}", mouse.name, mouse.is_expensive());
}