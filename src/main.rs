mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod util;

// use std::env::args;

fn main() {
    let mut solutions: Vec<(fn(&str, bool), bool)> = Vec::new();
    solutions.push((day1::run, false));
    solutions.push((day2::run, false));
    solutions.push((day3::run, false));
    solutions.push((day4::run, false));
    solutions.push((day5::run, true));
    solutions.push((day6::run, false));
    solutions.push((day7::run, false));
    solutions.push((day8::run, true));
    solutions.push((day9::run, false));

    for (day, (func, skip)) in solutions.iter().enumerate() {
        let day = day + 1;
        let input = format!("./inputs/day{}.txt", day);
        println!("----- day {} -----", day);
        if *skip {
            println!("... skip");
            continue;
        }

        func(&input, false);
        func(&input, true);
    }
}
