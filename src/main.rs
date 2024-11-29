use std::collections::HashMap;
use std::io;

#[derive(Debug)]
struct Item {
    name: String,
    quantity: i32,
    price: f32,
}

struct Inventory {
    items: HashMap<String, Item>,
}

impl Inventory {
    fn new() -> Self {
        Inventory {
            items: HashMap::new(),
        }
    }

    fn add_item(&mut self, name: String, quantity: i32, price: f32) {
        let item = Item {
            name: name.clone(),
            quantity,
            price,
        };
        self.items.insert(name, item);
    }

    fn remove_item(&mut self, name: &str) {
        self.items.remove(name);
    }

    fn list_items(&self) {
        println!("Available Items:");
        let mut sorted_items: Vec<_> = self.items.values().collect();
        sorted_items.sort_by(|a, b| a.name.cmp(&b.name));

        for (index, item) in sorted_items.iter().enumerate() {
            println!(
                "{}. {} - Quantity: {}, Price: ${:.2}",
                index + 1,
                item.name,
                item.quantity,
                item.price
            );
        }
    }

    fn get_item_by_number(&self, number: usize) -> Option<String> {
        let mut sorted_items: Vec<_> = self.items.keys().collect();
        sorted_items.sort();
        sorted_items.get(number - 1).cloned().cloned()
    }

    fn purchase_item(&mut self, name: &str, quantity: i32) {
        match self.items.get_mut(name) {
            Some(item) => {
                if item.quantity >= quantity {
                    item.quantity -= quantity;
                    println!(
                        "Purchased {} x {} - Total: ${:.2}",
                        quantity,
                        item.name,
                        item.price * quantity as f32
                    );
                } else {
                    println!(
                        "Not enough {} in stock. Available: {}",
                        item.name, item.quantity
                    );
                }
            }
            None => println!("Item not found!"),
        }
    }

    fn restock_item(&mut self, name: &str, quantity: i32) {
        match self.items.get_mut(name) {
            Some(item) => {
                item.quantity += quantity;
                println!(
                    "Restocked {} x {}. New quantity: {}",
                    quantity, item.name, item.quantity
                );
            }
            None => println!("Item not found!"),
        }
    }

    fn generate_report(&self) {
        println!("Inventory Report:");
        for item in self.items.values() {
            println!(
                "{} - Quantity: {}, Price: ${:.2}",
                item.name, item.quantity, item.price
            );
        }
    }
}

fn parse_input(input: &str) -> Result<i32, &str> {
    match input.trim().parse::<i32>() {
        Ok(num) => Ok(num),
        Err(_) => Err("Invalid input. Please enter a valid number."),
    }
}

fn main() {
    let mut inventory = Inventory::new();

    inventory.add_item("Biscuit".to_string(), 30, 15.00);
    inventory.add_item("Burger".to_string(), 10, 75.00);
    inventory.add_item("Pizza".to_string(), 15, 130.00);
    inventory.add_item("Shawarma".to_string(), 25, 25.00);
    inventory.add_item("Jellof Rice".to_string(), 20, 30.00);

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
                inventory.list_items();
                println!("Enter the number of the item to purchase:");
                let mut number = String::new();
                io::stdin().read_line(&mut number).expect("Failed to read line");
                let number: usize = match number.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid input. Please enter a number.");
                        continue;
                    }
                };

                if let Some(name) = inventory.get_item_by_number(number) {
                    println!("Enter quantity:");
                    let mut quantity = String::new();
                    io::stdin().read_line(&mut quantity).expect("Failed to read line");
                    let quantity: i32 = match parse_input(&quantity) {
                        Ok(num) => num,
                        Err(err) => {
                            println!("{}", err);
                            continue;
                        }
                    };

                    inventory.purchase_item(&name, quantity);
                } else {
                    println!("Invalid item number.");
                }
            }
            3 => {
                inventory.list_items();
                println!("Enter the number of the item to restock:");
                let mut number = String::new();
                io::stdin().read_line(&mut number).expect("Failed to read line");
                let number: usize = match number.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid input. Please enter a number.");
                        continue;
                    }
                };

                if let Some(name) = inventory.get_item_by_number(number) {
                    println!("Enter quantity to restock:");
                    let mut quantity = String::new();
                    io::stdin().read_line(&mut quantity).expect("Failed to read line");
                    let quantity: i32 = match parse_input(&quantity) {
                        Ok(num) => num,
                        Err(err) => {
                            println!("{}", err);
                            continue;
                        }
                    };

                    inventory.restock_item(&name, quantity);
                } else {
                    println!("Invalid item number.");
                }
            }
            4 => inventory.generate_report(),
            5 => break,
            _ => println!("Invalid choice. Please try again."),
        }
    }
}
