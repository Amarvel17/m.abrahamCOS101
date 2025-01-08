struct Laptop {
    brand: String,
    price: u32,
    quantity: u32
}
fn main() {
    let laptops = vec! [
        Laptop {brand: "HP".to_string(), price: 650_000, quantity: 3},
        Laptop {brand: "IBM".to_string(), price: 755_000, quantity: 3},
        Laptop {brand: "TOSHIBA".to_string(), price: 550_000, quantity: 3},
        Laptop {brand: "DELL".to_string(), price: 850_000, quantity: 3},
    ];

    let mut total_cost = 0;

    for laptop in &laptops {
        total_cost += laptop.price * laptop.quantity;
    }

    println!("The total cost for the customer is {}",total_cost);
}
