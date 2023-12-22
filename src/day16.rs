use std::collections::{HashMap, HashSet};

use once_cell::sync::Lazy;

use crate::util;

const U: i32 = 0;
const R: i32 = 1;
const D: i32 = 2;
const L: i32 = 3;

static DIR: Lazy<Vec<(i32, i32)>> = Lazy::new(|| {
    let mut ret = Vec::new();
    ret.push((-1, 0));
    ret.push((0, 1));
    ret.push((1, 0));
    ret.push((0, -1));
    ret
});

static CHANGE: Lazy<HashMap<(u8, i32), Vec<i32>>> = Lazy::new(|| {
    let mut ret: HashMap<_, _> = HashMap::new();

    ret.insert((b'/', U), vec![R]);
    ret.insert((b'/', R), vec![U]);
    ret.insert((b'/', D), vec![L]);
    ret.insert((b'/', L), vec![D]);

    ret.insert((b'\\', U), vec![L]);
    ret.insert((b'\\', L), vec![U]);
    ret.insert((b'\\', D), vec![R]);
    ret.insert((b'\\', R), vec![D]);

    ret.insert((b'-', U), vec![L, R]);
    ret.insert((b'-', D), vec![L, R]);

    ret.insert((b'|', R), vec![U, D]);
    ret.insert((b'|', L), vec![U, D]);

    ret
});

fn change(d: i32, s: u8) -> Vec<i32> {
    match CHANGE.get(&(s, d)) {
        None => vec![d],
        Some(val) => val.to_vec(),
    }
}

fn solve(raw: &Vec<Vec<u8>>, start: (i32, i32), d: i32) -> usize {
    let mut visit: HashSet<(i32, i32, i32)> = HashSet::new();

    let mut queue: Vec<(i32, i32, i32)> = Vec::new();
    queue.push((start.0, start.1, d));

    while !queue.is_empty() {
        let curr = queue.pop().unwrap();
        if curr.0 < 0 || curr.1 < 0 || curr.0 >= raw.len() as i32 || curr.1 >= raw[0].len() as i32 {
            continue;
        }

        if visit.contains(&curr) {
            continue;
        }

        visit.insert(curr);

        for c in change(curr.2, raw[curr.0 as usize][curr.1 as usize]) {
            let dir = DIR[c as usize];
            queue.push((curr.0 + dir.0, curr.1 + dir.1, c));
        }
    }

    let mut unique = HashSet::new();
    for q in visit {
        unique.insert((q.0, q.1));
    }

    unique.len()
}

pub fn run(filename: &str, adv: bool) {
    let mut raw: Vec<Vec<u8>> = Vec::new();
    if let Ok(lines) = util::read_lines(filename) {
        for (_, line) in lines.enumerate() {
            if let Ok(data) = line {
                for part in data.split(',') {
                    raw.push(part.as_bytes().to_vec());
                }
            }
        }
    }

    if !adv {
        println!("{}", solve(&raw, (0, 0), R));
        return;
    }

    let mut max = 0;
    for i in 0..raw.len() {
        max = max.max(solve(&raw, (i as i32, 0), R));
        max = max.max(solve(&raw, (i as i32, (raw[0].len() - 1) as i32), L));
    }

    for i in 0..raw[0].len() {
        max = max.max(solve(&raw, (0, i as i32), D));
        max = max.max(solve(&raw, ((raw.len() - 1) as i32, i as i32), U));
    }

    println!("{}", max);
}
