mod day1;
mod day2;
mod day3;
mod util;

use std::env;

fn main() {
    let mut solutions: Vec<fn(&str, bool)> = Vec::new();
    solutions.push(day1::run);
    solutions.push(day2::run);
    solutions.push(day3::run);

    for (day, func) in solutions.iter().enumerate() {
        let day = day + 1;
        let input = format!("./inputs/day{}.txt", day);
        println!("----- day {} -----", day);
        func(&input, false);
        func(&input, true);
    }
}
