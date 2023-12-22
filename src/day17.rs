use once_cell::sync::Lazy;

use crate::util;

static _URDL: Lazy<Vec<(i32, i32)>> = Lazy::new(|| {
    let mut ret = Vec::new();
    ret.push((-1, 0));
    ret.push((0, 1));
    ret.push((1, 0));
    ret.push((0, -1));
    ret
});

pub fn _node_code(i: i32, j: i32, d: i32) -> i32 {
    i * 256 * 256 + j * 256 + d
}

pub fn run(filename: &str, _: bool) {
    let mut raw: Vec<Vec<i32>> = Vec::new();
    if let Ok(lines) = util::read_lines(filename) {
        for (_, line) in lines.enumerate() {
            if let Ok(data) = line {
                for part in data.split(',') {
                    raw.push(part.bytes().map(|x| (x - b'0') as i32).collect());
                }
            }
        }
    }

    // create graph
    for _ in 0..raw.len() {
        for _ in 0..raw[0].len() {
            for _ in 0..4 {}
        }
    }
}
