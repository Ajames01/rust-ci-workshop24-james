fn main() {
    let mut inventory = Inventory::new();

    inventory.add_item("Biscuit".to_string(), 5, 1500.0);
    inventory.add_item("Burger".to_string(), 10, 7500.0);
    inventory.add_item("Pizza".to_string(), 3, 13000.0);
    inventory.add_item("Shawarma".to_string(), 3, 2500.0);
    inventory.add_item("Jellof Rice".to_string(), 3, 3000.0);

    loop {
        println!("\nMenu:");
        println!("1. List items");
        println!("2. Purchase item");
        println!("3. Restock item");
        println!("4. Generate report");
        println!("5. Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        let choice: i32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number.");
                continue;
            }
        };

        match choice {
            1 => inventory.list_items(),
            2 => {
                let mut name = String::new();
                let mut quantity = String::new();

                println!("Enter item name:");
                io::stdin().read_line(&mut name).expect("Failed to read line");
                let name = name.trim();

                if !inventory.items.contains_key(name) {
                    println!("Item not found!");
                    continue;
                }

                println!("Enter quantity:");
                io::stdin().read_line(&mut quantity).expect("Failed to read line");
                let quantity: i32 = match parse_input(&quantity) {
                    Ok(num) => num,
                    Err(err) => {
                        println!("{}", err);
                        continue;
                    }
                };

                inventory.purchase_item(name, quantity);
            },
            3 => {
                let mut name = String::new();
                let mut quantity = String::new();

                println!("Enter item name:");
                io::stdin().read_line(&mut name).expect("Failed to read line");
                let name = name.trim();

                if !inventory.items.contains_key(name) {
                    println!("Item not found!");
                    continue;
                }

                println!("Enter quantity to restock:");
                io::stdin().read_line(&mut quantity).expect("Failed to read line");
                let quantity: i32 = match parse_input(&quantity) {
                    Ok(num) => num,
                    Err(err) => {
                        println!("{}", err);
                        continue;
                    }
                };

                inventory.restock_item(name, quantity);
            },
            4 => inventory.generate_report(),
            5 => break,
            _ => println!("Invalid choice. Please try again."),
        }
    }
}
