// Given a list of integers, use a vector and return the mean, median and mode of the list
use std::fs;
use std::path::Path;

fn main() {
    let filename = "./data/integers.txt";

    let file_path = Path::new(filename);

    println!("In file {}", filename);

    let contents = fs::read_to_string(file_path)
        .expect("Something went wrong reading the file");

    let split_contents = contents.split_whitespace();

    print!("Here's the contents: {{ ");
    for item in split_contents {
        print!("{}, ", item);
    }
    print!("}}");
}
