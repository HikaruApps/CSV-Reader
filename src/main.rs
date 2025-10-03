use std::io::{self, Write};

fn main() {
    println!("Main menu:");
    println!("add - create a new CSV file");
    println!("edit - edit a CSV file");
    println!("read - read a CSV file");
    println!("remove - remove a CSV file");

    let mut choice = String::new();
    println!("Enter your choice: ");
    io::stdin().read_line(&mut choice).unwrap();
    let choice = choice.trim();

    match choice {
        "add" => println!("You chose to create a new CSV file"),
        "edit" => println!("You chose to edit a CSV file"),
        "read" => println!("You chose to read a CSV file"),
        "remove" => println!("You chose to remove a CSV file"),
        _ => println!("Unknown choice."),
    }
}
