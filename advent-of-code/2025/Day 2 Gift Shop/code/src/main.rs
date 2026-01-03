// https://adventofcode.com/2025/day/2

#[cfg(test)]
mod tests;

use std::io::Error;
use std::{env, fs};

fn main() {
    println!("Starting...");

    let ranges   = get_ranges();

    let result = search_ranges(ranges);

    println!("Total dups= {result}");
}

pub fn search_ranges(ranges: Vec<Range>)-> i64{
    let mut total_dups: i64 = 0;

    for range in ranges.iter(){
        let cur_range_dups = get_dups_for_range(range);
        total_dups += cur_range_dups;
    }

    return total_dups;
}
fn get_dups_for_range(range: &Range)-> i64{
    let mut dups_in_range = 0;

    // First pass naive solution
    // Should be optimized later
    for n in range.start..=range.end {
        let is_valid = num_is_valid(&n);

        if !is_valid{
            dups_in_range += n;
        }
    }

    
    return dups_in_range;
}

fn num_is_valid(num: &i64)-> bool{
    //return false;
    let num_string = num.to_string();

    let num_count = num_string.chars().count();

    // value is odd in size, cannot be mirrored
    if num_count %2 != 0{
        return true;
    }

    let halfway_index = num_count / 2;

    let num_string_half = &num_string[0..halfway_index];

    let mut duped_first_half: String = num_string_half.to_string();
    duped_first_half.push_str(num_string_half);

    if num_string == duped_first_half {
        return false;
    }
    else {return true;}

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

        let start_val = values[0].parse::<i64>().unwrap();
        let end_val =  values[1].parse::<i64>().unwrap();

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
    start: i64,
    end: i64
}