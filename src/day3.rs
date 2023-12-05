use crate::util;
use once_cell::sync::Lazy;
use std::collections::{HashMap, HashSet};

static DIR: Lazy<Vec<(i32, i32)>> = Lazy::new(|| {
    let mut ret = Vec::new();
    for i in 0..3 {
        for j in 0..3 {
            if !(i == 1 && j == 1) {
                ret.push((i - 1, j - 1));
            }
        }
    }
    ret
});

static NON_SYMBOL: Lazy<HashSet<char>> = Lazy::new(|| {
    let mut ret = HashSet::new();
    for c in 0..10 {
        ret.insert(char::from_digit(c, 10).unwrap());
    }
    ret.insert('.');
    ret
});

fn compute(lines: &Vec<String>, adv: bool) {
    let mut code: i32 = 0;

    let mut sym: HashSet<(i32, i32)> = HashSet::new();
    let mut sss: HashSet<(i32, i32)> = HashSet::new();
    for (i, line) in lines.iter().enumerate() {
        for (j, ch) in line.chars().enumerate() {
            if !NON_SYMBOL.contains(&ch) {
                sym.insert((i as i32, j as i32));
            }
            if ch == '*' {
                sss.insert((i as i32, j as i32));
            }
        }
    }

    let mut gears: HashMap<(i32, i32), Vec<i32>> = HashMap::new();
    let mut stars: HashSet<(i32, i32)> = HashSet::new();

    for (i, line) in lines.iter().enumerate() {
        let mut p = 0;
        let chars = Vec::from(line.as_str());

        while p < chars.len() {
            let mut l = 0;
            let mut v: i32 = 0;
            let mut b = false;

            stars.clear();
            while l + p < chars.len() && chars[l + p].is_ascii_digit() {
                for d in DIR.iter() {
                    let pos = (i as i32 + d.0, (l + p) as i32 + d.1);
                    if sym.contains(&pos) {
                        b = true;
                    }
                    if sss.contains(&pos) {
                        stars.insert(pos);
                    }
                }

                v = 10 * v + (chars[l + p] - b'0') as i32;
                l += 1;
            }

            if b {
                code += v;
                for star in stars.iter() {
                    gears.entry((star.0, star.1)).or_insert(Vec::new()).push(v);
                }
            }

            p += l + 1;
        }
    }

    let mut gc = 0;
    for (&_, v) in gears.iter() {
        if v.len() == 2 {
            gc += v.get(0).unwrap() * v.get(1).unwrap();
        }
    }

    println!("{}", if adv { gc } else { code });
}

pub fn run(filename: &str, adv: bool) {
    let mut data = Vec::new();
    if let Ok(lines) = util::read_lines(filename) {
        for line in lines {
            if let Ok(str) = line {
                data.push(str);
            }
        }
    }

    compute(&data, adv);
}
