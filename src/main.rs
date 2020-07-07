/// Given a list of integers, use a vector and return the mean, median and mode of the list

use std::{env, fs};
use std::path::Path;
use ::statistics::averages;

fn main() {
    let file_path = Path::new("./data/integers.txt");

    let mut contents: String = String::new();

    // Read the contents of the file to the variable contents.
    // The match statement is all just error handling really: First it tries to read the directory:
    match fs::read_to_string(file_path) {
        Ok(content_string) => {
            contents.push_str(content_string.as_str());
        }
        // If that fails it will print out the error, plus some helpers to point out what the
        // variables such as file_path and our cwd that we are working with are.
        Err(error) => {
            eprintln!("{}", error);
            println!("Path was: {}", file_path.display());
            println!("Current working directory is: {:?}", match env::current_dir() {
                Ok(dir) => match dir.to_str() {
                    Some(string) => String::from(string),
                    None => String::from("Apparently none!"),
                },
                // And here will only be reached if there's some sort of problem with the cwd...
                Err(_error) => {
                    eprintln!("We even have a problem getting cwd!");
                    String::from("Apparently none!")
                }
            })
        }
    };

    let split_contents = contents.split_whitespace();
    let mut contents_vec: Vec<i32> = Vec::new();

    for item in split_contents {
        match item.parse::<i32>() {
            Ok(value) => {
                contents_vec.push(value);
            },
            Err(_) => {
                eprintln!("String {} doesn't parse into an int!", item);
            }
        }
    }

    println!("\nHere's our vector:\n{:?}", contents_vec);
    println!("\nThe mean of the values is: {}", averages::mean(&contents_vec));

    match averages::median(&contents_vec) {
        Some(median) => {
            println!("\nThe median of the values is: {}", median);
        },
        None => {
            println!("\nThe median couldn't be found, probably cause the list was empty!");
        }
    }

    match averages::mode(&contents_vec) {
        Some(mode) => {
            println!("\nThe mode of the values is: {}", mode);
        },
        None => {
            println!("The mode couldn't be found, probably cause the list was empty!");
        }
    }

    println!();
}
