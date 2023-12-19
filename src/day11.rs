use std::collections::HashSet;

use crate::util;

fn recalc(coord: i32, map: &HashSet<i32>, adv: bool) -> i32 {
    let mut empty = 0;
    for i in 0..coord {
        if !map.contains(&i) {
            empty += 1;
        }
    }

    if adv {
        empty *= 999999;
    }

    empty + coord
}

pub fn run(filename: &str, adv: bool) {
    let mut stars: Vec<(i32, i32)> = Vec::new();
    let mut rows = HashSet::new();
    let mut cols = HashSet::new();

    if let Ok(lines) = util::read_lines(filename) {
        for (row, line) in lines.enumerate() {
            if let Ok(data) = line {
                for (col, sym) in data.bytes().enumerate() {
                    if sym == b'#' {
                        rows.insert(row as i32);
                        cols.insert(col as i32);
                        stars.push((row as i32, col as i32));
                    }
                }
            }
        }
    }

    let mut expanse = Vec::new();
    for star in stars {
        let new = (recalc(star.0, &rows, adv), recalc(star.1, &cols, adv));
        expanse.push(new);
    }

    let mut sum: i64 = 0;
    for i in 0..expanse.len() {
        for j in i + 1..expanse.len() {
            sum +=
                ((expanse[i].0 - expanse[j].0).abs() + (expanse[i].1 - expanse[j].1).abs()) as i64;
        }
    }

    println!("{}", sum)
}
