use crate::input_parsing::Range;

pub fn get_invs_for_ranges(ranges: Vec<Range>)-> i64{
    let mut total_dups: i64 = 0;

    for range in ranges.iter(){
        let cur_range_dups = get_dups_for_range(range);
        total_dups += cur_range_dups;
    }

    return total_dups;
}
pub fn get_dups_for_range(range: &Range)-> i64{
    let mut dups_in_range = 0;

    // First pass naive solution
    // Should be optimized later
    for n in range.start..=range.end {
        let is_valid = is_valid_loop(&n);

        if !is_valid{
            dups_in_range += n;
        }
    }

    
    return dups_in_range;
}

/** 
 * Get if number is valid by looping through possible mirror lengths
 */
fn is_valid_loop(num: &i64)-> bool{
    let num_string = num.to_string();

    let num_count = num_string.chars().count();
    let halfway_index = num_count / 2;

    let mut i = 1;
    while i <= halfway_index {

        // length not a factor of num_count
        if num_count % i != 0{
            i += 1;
            continue;
        }

        let num_sample = &num_string[0..i];
        let reps = num_count / i;
        
        let repeated_sample = num_sample.repeat(reps as usize);

        if repeated_sample == num_string{
            return false;
        }

        i += 1;
    }

    return true;
}

// fn num_is_valid(num: &i64)-> bool{
//     let num_string = num.to_string();

//     let num_count = num_string.chars().count();

//     let halfway_index = num_count / 2;

//     let num_string_half = &num_string[0..halfway_index];

//     let mut duped_first_half: String = num_string_half.to_string();
//     duped_first_half.push_str(num_string_half);

//     if num_string == duped_first_half {
//         return false;
//     }
//     else {return true;}
// }