use std::collections::HashMap;

use crate::util;

fn bit_count(mask: u64) -> i64 {
    let mut c = 0;
    for i in 0..64 {
        if (mask & (1_u64 << i)) != 0 {
            c += 1;
        }
    }

    c
}

fn crunch(input: &Vec<u64>, end: usize) -> i64 {
    let len = end.min(input.len() - end);
    let mut c = 0;
    for i in 0..len {
        let b = bit_count(input[end - i - 1] ^ input[end + i]);
        c += b;
    }

    c
}

fn solve(input: &Vec<String>, adv: bool) -> i64 {
    let input: Vec<Vec<u8>> = input.iter().map(|x| x.clone().into_bytes()).collect();
    let mut hor: Vec<u64> = Vec::new();
    let mut ver: Vec<u64> = Vec::new();

    let m: usize = input.len();
    let n: usize = input[0].len();

    for i in 0..m {
        let mut mask: u64 = 0;
        for j in 0..n {
            if input[i][j] == b'#' {
                mask |= 1_u64 << j;
            }
        }
        hor.push(mask);
    }

    for j in 0..n {
        let mut mask: u64 = 0;
        for i in 0..m {
            if input[i][j] == b'#' {
                mask |= 1_u64 << i;
            }
        }
        ver.push(mask);
    }

    let defect = if adv { 1 } else { 0 };
    for end in 1..hor.len() {
        if crunch(&hor, end) == defect {
            return 100 * end as i64;
        }
    }

    for end in 1..ver.len() {
        if crunch(&ver, end) == defect {
            return end as i64;
        }
    }

    0
}

pub fn run(filename: &str, adv: bool) {
    let mut raw = Vec::new();
    if let Ok(lines) = util::read_lines(filename) {
        for (_, line) in lines.enumerate() {
            if let Ok(data) = line {
                raw.push(data)
            }
        }
    }

    let mut tasks = Vec::new();
    let mut curr = 0;
    while curr < raw.len() {
        let mut i = 0;
        while curr + i < raw.len() && !raw[curr + i].is_empty() {
            i += 1;
        }

        let mut task: Vec<String> = Vec::new();
        for j in 0..i {
            task.push(raw[curr + j].clone());
        }

        tasks.push(task);
        curr += i + 1;
    }

    let mut sum: i64 = 0;
    for task in tasks.iter() {
        let result = solve(task, adv);
        sum += result;
    }

    println!("= {}", sum);
}
