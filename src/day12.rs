use std::collections::HashMap;

use crate::util;

struct Task {
    data: Vec<u8>,
    size: Vec<usize>,
}

fn solve(
    memo: &mut HashMap<(usize, usize), i64>,
    data: &Vec<u8>,
    size: &Vec<usize>,
    pos_data: usize,
    pos_size: usize,
) -> i64 {
    let key = (pos_data, pos_size);
    if let Some(cached) = memo.get(&key) {
        return *cached;
    }

    // tail
    if pos_size == size.len() {
        for i in pos_data..data.len() {
            if data[i] == b'#' {
                memo.insert(key, 0);
                return 0;
            }
        }

        memo.insert(key, 1);
        return 1;
    }

    let mut sum = 0;

    // space
    if pos_data < data.len() && data[pos_data] != b'#' {
        sum += solve(memo, data, size, pos_data + 1, pos_size);
    }

    // island
    if pos_data + size[pos_size] <= data.len() {
        let mut m = true;
        for i in 0..size[pos_size] {
            if data[pos_data + i] == b'.' {
                m = false;
            }
        }

        if m {
            if pos_data + size[pos_size] == data.len() {
                sum += solve(memo, data, size, pos_data + size[pos_size], pos_size + 1);
            } else if data[pos_data + size[pos_size]] != b'#' {
                sum += solve(
                    memo,
                    data,
                    size,
                    pos_data + size[pos_size] + 1,
                    pos_size + 1,
                );
            }
        }
    }

    memo.insert(key, sum);
    sum
}

fn x5<T>(src: &Vec<T>, addon: &Option<T>) -> Vec<T>
where
    T: Copy,
{
    let mut ret = Vec::new();
    for i in 0..5 {
        for t in src {
            ret.push(*t);
        }

        if i < 4 {
            if let Some(val) = addon {
                ret.push(*val);
            }
        }
    }

    ret
}

pub fn run(filename: &str, adv: bool) {
    let mut tasks = Vec::new();
    if let Ok(lines) = util::read_lines(filename) {
        for (_, line) in lines.enumerate() {
            if let Ok(data) = line {
                if let Some((data, size)) = data.split_once(' ') {
                    tasks.push(Task {
                        data: data.bytes().collect(),
                        size: size.split(',').map(|x| x.parse().unwrap()).collect(),
                    })
                }
            }
        }
    }

    let mut sum: i64 = 0;
    for t in tasks {
        let mut memo: HashMap<(usize, usize), i64> = HashMap::new();

        let (data, size) = if adv {
            (x5(&t.data, &Some(b'?')), x5(&t.size, &None))
        } else {
            (t.data, t.size)
        };

        let result = solve(&mut memo, &data, &size, 0, 0);
        sum += result;
    }

    println!("= {}", sum);
}
