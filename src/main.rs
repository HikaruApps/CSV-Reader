use std::fs;
use std::env;
use std::path::Path;
use std::fs::File;
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
        "add" => add(),
        "edit" => println!("You chose to edit a CSV file"),
        "read" => println!("You chose to read a CSV file"),
        "remove" => println!("You chose to remove a CSV file"),
        _ => println!("Unknown choice."),
    }
}

fn add() {

    let folder_path = Path::new("./data");

    if !folder_path.exists() {
        fs::create_dir_all(folder_path).unwrap();
        println!("Creating folder {}", folder_path.display());
    }

    println!("Enter the name of the csv file");
    let mut filename = String::new();
    io::stdin().read_line(&mut filename).unwrap();
    let filename = filename.trim();

    let file_path = Path::new(folder_path).join(filename);

    if filename.is_empty() {
        println!("The file name cannot be empty");
        return;
    }

    match File::create(file_path) {

        Ok(_) => println!("Created file {}", filename),
        Err(_) => println!("Could not create file {}", filename),
    }
}