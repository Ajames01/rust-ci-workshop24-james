use std::collections::HashMap;
use std::io::{self, Write};

#[derive(Debug)]
struct Item {
    name: String,
    quantity: i32,
    price: f32, // Price in Naira
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
        let item = Item { name: name.clone(), quantity, price };
        self.items.insert(name, item);
    }

    fn remove_item(&mut self, name: &str) {
        self.items.remove(name);
    }

    fn list_items(&self) {
        for item in self.items.values() {
            println!("{:?}", item);
        }
    }

    fn purchase_item(&mut self, name: &str, quantity: i32) {
        match self.items.get_mut(name) {
            Some(item) => {
                if item.quantity >= quantity {
                    item.quantity -= quantity;
                    println!("Purchased {} x {} - Total: ₦{:.2}", quantity, item.name, item.price * quantity as f32);
                } else {
                    println!("Not enough {} in stock. Available: {}", item.name, item.quantity);
                }
            },
            None => println!("Item not found!"),
        }
    }

    fn restock_item(&mut self, name: &str, quantity: i32) {
        match self.items.get_mut(name) {
            Some(item) => {
                item.quantity += quantity;
                println!("Restocked {} x {}. New quantity: {}", quantity, item.name, item.quantity);
            },
            None => println!("Item not found!"),
        }
    }

    fn generate_report(&self) {
        println!("Inventory Report:");
        for item in self.items.values() {
            println!("{:?}", item);
        }
    }

    fn check_item(&self, name: &str) -> bool {
        self.items.contains_key(name)
    }
}

fn authenticate_user(username: &str, password: &str) -> bool {
    let valid_username = "admin";
    let valid_password = "password";
    username == valid_username && password == valid_password
}

fn parse_input(input: &str) -> Result<i32, &str> {
    match input.trim().parse::<i32>() {
        Ok(num) => Ok(num),
        Err(_) => Err("Invalid input. Please enter a valid number."),
    }
}

fn main() {
    let mut inventory = Inventory::new();

    // Adding items with prices converted to Naira at the rate of 1 USD = 1800 NGN
    inventory.add_item("Biscuit".to_string(), 5,1200.0); // ₦1200.00
    inventory.add_item("Burger".to_string(), 10, 8000.0); // ₦8000.00
    inventory.add_item("Pizza".to_string(), 3, 10000.0);   // ₦10000.00
    inventory.add_item("Shawarma".to_string(), 3, 2500.0); // ₦2500.00
    inventory.add_item("Jellof Rice".to_string(), 3, 3000.0); // ₦3000.00

    let mut username = String::new();
    let mut password = String::new();

    println!("Enter username:");
    io::stdin().read_line(&mut username).expect("Failed to read line");
    
    println!("Enter password:");
    io::stdin().read_line(&mut password).expect("Failed to read line");

    if !authenticate_user(username.trim(), password.trim()) {
        println!("Authentication failed. Exiting...");
        return;
    }

    loop {
        println!("\nMenu:");
        println!("1. List items");
        println!("2. Purchase item");
        println!("3. Restock item");
        println!("4. Generate report");
        println!("5. Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        
        let choice: i32 = match parse_input(&choice) {
            Ok(num) => num,
            Err(err) => {
                println!("{}", err);
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

                if !inventory.check_item(name) {
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

                if !inventory.check_item(name) {
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