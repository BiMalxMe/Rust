fn main() {
    let items = vec![
        "Apple", "Banana", "Cherry", "Date", "Elderberry",
        "Fig", "Grape", "Honeydew", "Indian Fig", "Jackfruit",
    ];

    let offset = 0; // Start showing from the 1th item (index 0)
    let limit = 3;  // Show only 3 items

    let visible_items = items.iter()
        .skip(9)
        .take(limit);

    println!("Visible window (offset={}, limit={}):", offset, limit);
    for item in visible_items {
        println!("- {}", item);
    }
}
