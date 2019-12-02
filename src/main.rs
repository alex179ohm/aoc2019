#![allow(dead_code)]
mod day1;
mod day2;
mod day3;

use std::fs;

fn get_input_by_day(n: i32) -> String {
    fs::read_to_string(format!("inputs/{}.txt", n)).expect("failed to read input file")
}

fn main() {
//    let input = get_input_by_day(1);
//    println!(
//        "day 1: {} {}",
//        day1::one(input.clone()),
//        day1::two(input)
//    );
    let input = get_input_by_day(2);
    println!(
        "day 2: {} {}",
        day2::one(input.clone()),
        day2::two(input)
    );
    let input = get_input_by_day(3);
    println!(
        "day 3: {} {}",
        day3::one(input.clone()),
        day3::two(input)
    );
}

