// https://adventofcode.com/2025/day/2

mod tests;

use std::io::Error;
use std::{env, fs};

fn main() {
    println!("Hello, world!");

    let ranges = get_ranges();

    let result: i32 = search_ranges(ranges);

    println!("Total dups= {result}");
}

pub fn search_ranges(ranges: Vec<Range>)-> i32{
    let mut total_dups: i32 = 0;

    for range in ranges.iter(){
        let cur_range_dups = get_dups_for_range(range);
        total_dups += cur_range_dups;
    }

    return total_dups;
}
fn get_dups_for_range(range: &Range)-> i32{
    let mut dups_in_range = 0;

    dups_in_range += 2;
    
    return dups_in_range;
}

fn get_ranges()-> Vec<Range> {
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

        let start_val = values[0].parse::<i32>().unwrap();
        let end_val =  values[1].parse::<i32>().unwrap();

        results.push(Range{start: start_val, end: end_val});
    }
    
    return results;
}

fn get_input()-> Result<String, Error>{
    let cur_path = env::current_dir()?;
    println!("current directory is {}", cur_path.display());

    let file_path = cur_path.join("input.txt");
    println!("reading file at {}", file_path.display());

    let contents = fs::read_to_string(&file_path)?;
    
    Ok(contents)
}

pub struct Range {
    start: i32,
    end: i32
}