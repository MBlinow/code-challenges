// https://adventofcode.com/2025/day/2

use std::{fs, path::Path};

fn main() {
    println!("Hello, world!");

    let file_contents = get_input();

    println!("file_contents: {file_contents}")
}

fn get_input()-> String {
    //let file_path = "/input";
    let path: &Path = Path::new("./input");

    let contents: String = fs::read_to_string(path)    
        .expect("Should have been able to read the file");
    
    return contents;
}