use std::io;

fn main() {
    // Menu with prices
    let menu = [
        ("P", "Poundo Yam/Edikaiko Soup", 3200),
        ("F", "Fried Rice & Chicken", 3000),
        ("A", "Amala & Ewedu Soup", 2500),
        ("E", "Eba & Egusi Soup", 2000),
        ("W", "White Rice & Stew", 2500),
    ];

    let mut total_amount = 0;

    loop {
        // Display menu
        println!("Menu:");
        for (code, name, price) in &menu {
            println!("{} = {} - ₦{}", code, name, price);
        }

        // Input food type
        println!("\nEnter the code of the food item (or 'Q' to quit):");
        let mut food_code = String::new();
        io::stdin().read_line(&mut food_code).unwrap();
        let food_code = food_code.trim().to_uppercase();

        // Exit condition
        if food_code == "Q" {
            break;
        }

        // Find the food item
        let food_item = menu.iter().find(|&&(code, _, _)| code == food_code);
        if let Some((_, name, price)) = food_item {
            println!("You selected: {} - ₦{}", name, price);

            // Input quantity
            println!("Enter the quantity:");
            let mut quantity_str = String::new();
            io::stdin().read_line(&mut quantity_str).unwrap();
            let quantity: i32 = quantity_str.trim().parse().unwrap_or(0);

            if quantity > 0 {
                let item_total = price * quantity;
                total_amount += item_total;
                println!(
                    "Added {} x {} to your order. Subtotal: ₦{}\n",
                    quantity, name, item_total
                );
            } else {
                println!("Invalid quantity entered. Try again.\n");
            }
        } else {
            println!("Invalid food code. Try again.\n");
        }
    }

    
}
