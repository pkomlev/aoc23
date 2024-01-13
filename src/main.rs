mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day2;
mod day20;
mod day21;
mod day22;
mod day23;
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
    solutions.push((day10::run, false));
    solutions.push((day11::run, false));
    solutions.push((day12::run, false));
    solutions.push((day13::run, false));
    solutions.push((day14::run, false));
    solutions.push((day15::run, false));
    solutions.push((day16::run, false));
    solutions.push((day17::run, false));
    solutions.push((day18::run, false));
    solutions.push((day19::run, false));
    solutions.push((day20::run, true));
    solutions.push((day21::run, false));
    solutions.push((day22::run, false));
    solutions.push((day23::run, false));

    for (day, (func, skip)) in solutions.iter().enumerate() {
        let day = day + 1;

        let input = format!("./inputs/day{}.txt", day);
        println!("----- day {} -----", day);

        if !util::file_exists(&input) {
            println!("... no input");
            continue;
        }

        if *skip {
            println!("... skip");
            continue;
        }

        func(&input, false);
        func(&input, true);
    }
}
