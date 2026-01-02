// https://adventofcode.com/2025/day/2

use std::io::Error;
use std::{fs, env};

fn main() {
    println!("Hello, world!");

    let file_contents: String = get_input().unwrap();

    println!("file_contents: {file_contents}")
}

// fn get_ranges()-> Result<Vec<Range>, Error>{
//     let file_contents: String = get_input().unwrap();

//     let raw_ranges: Vec<String> = file_contents.split(",").collect::<Vec<String>>();

//     println!("Collected ranges are {raw_ranges}");


//     let results: Vec<Range> = Vec::with_capacity(raw_ranges.count());
//     for i in raw_ranges.iter_mut(){
//         values = i.split('-');
//         //results.push({start: values[0] end: values[1]});
//     }

// }

fn get_input()-> Result<String, Error>{
    let cur_path = env::current_dir()?;
    println!("current directory is {}", cur_path.display());

    let file_path = cur_path.join("input.txt");
    println!("reading file at {}", file_path.display());

    let contents = fs::read_to_string(&file_path)?;
    
    Ok(contents)
}

struct Range {
    start: i32,
    end: i32
}