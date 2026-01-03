use std::io::Error;
use std::{env, fs};

fn get_input()-> Result<String, Error>{
    let cur_path = env::current_dir()?;
    println!("current directory is {}", cur_path.display());

    let file_path = cur_path.join("input.txt");
    println!("reading file at {}", file_path.display());

    let contents = fs::read_to_string(&file_path)?;
    
    Ok(contents)
}

pub fn get_ranges()-> Vec<Range> {
    let file_contents: String = get_input().unwrap();

    let raw_ranges: Vec<String> = file_contents
    .split(",")
    .map(|s| s.to_string())
    .collect::<Vec<String>>();

    let mut results: Vec<Range> = Vec::with_capacity(raw_ranges.len());

    for i in raw_ranges.iter(){
        let values = i.split('-')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        let start_val = values[0].parse::<i64>().unwrap();
        let end_val =  values[1].parse::<i64>().unwrap();

        results.push(Range{start: start_val, end: end_val});
    }
    
    return results;
}

pub struct Range {
    pub start: i64,
    pub end: i64
}