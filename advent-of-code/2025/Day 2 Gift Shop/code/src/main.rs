// https://adventofcode.com/2025/day/2

mod part1;
mod part2;

mod input_parsing;

use crate::input_parsing::{get_ranges};

fn main() {
    println!("Starting...");

    let part2 = true;

    let ranges   = get_ranges();

    let result: i64;

    if part2{
        result = part2::get_invs_for_ranges(ranges);
    } else {
        result = part1::get_invs_for_ranges(ranges);
    }

    println!("Total dups= {result}");
}
