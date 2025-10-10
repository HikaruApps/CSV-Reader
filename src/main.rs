use std::fs;
use std::env;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::{self, Write};
use csv::Reader;
use std::error::Error;

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
        "edit" => edit(),
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

    let mut file_path = folder_path.join(filename);

    if file_path.extension() == None {
        file_path.set_extension("csv");
    }

    if filename.is_empty() {
        println!("The file name cannot be empty");
        return;
    }

    if !file_path.exists() {
        match File::create(&file_path) {

            Ok(_) => println!("Created file {}.csv", file_path.display()),
            Err(e) => println!("Could not create file {}.csv", e),
        }
    } else { println!("File {} already exists", filename) }

}

fn edit() {

    let file = File::open("./data").unwrap();
    let mut csv_reader = Reader::from_reader(file);
}