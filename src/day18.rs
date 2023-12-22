use std::collections::HashMap;

use once_cell::sync::Lazy;
use regex::Regex;

use crate::util;

static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\w) (\d+) \(#(\w+)\)").unwrap());

static DIR: Lazy<HashMap<&str, (i32, i32)>> = Lazy::new(|| {
    let mut ret = HashMap::new();
    ret.insert("U", (-1, 0));
    ret.insert("D", (1, 0));
    ret.insert("R", (0, 1));
    ret.insert("L", (0, -1));
    ret
});

static URDL: Lazy<Vec<(i32, i32)>> = Lazy::new(|| {
    let mut ret = Vec::new();
    ret.push((-1, 0));
    ret.push((0, 1));
    ret.push((1, 0));
    ret.push((0, -1));
    ret
});

pub fn run(filename: &str, _: bool) {
    let mut raw: Vec<(String, i32, String)> = Vec::new();
    if let Ok(lines) = util::read_lines(filename) {
        for (_, line) in lines.enumerate() {
            if let Ok(data) = line {
                if let Some(matches) = RE.captures(&data) {
                    let dir = matches.get(1).map(|x| x.as_str()).unwrap().to_owned();
                    let num: i32 = matches.get(2).map(|x| x.as_str()).unwrap().parse().unwrap();
                    let code = matches.get(3).map(|x| x.as_str()).unwrap().to_owned();
                    raw.push((dir, num, code));
                }
            }
        }
    }

    let mut dig: HashMap<(i32, i32), i32> = HashMap::new();
    let mut pos = (0, 0);
    dig.insert(pos, 1);

    for r in raw {
        let dir = DIR[&r.0 as &str];
        for _ in 0..(r.1 as usize) {
            pos.0 += dir.0;
            pos.1 += dir.1;
            dig.insert(pos, 1);
        }
    }

    let m_lo = dig.keys().map(|x| x.0).min().unwrap();
    let m_hi = dig.keys().map(|x| x.0).max().unwrap();
    let n_lo = dig.keys().map(|x| x.1).min().unwrap();
    let n_hi = dig.keys().map(|x| x.1).max().unwrap();

    let m = (m_hi - m_lo + 1 + 2) as usize;
    let n = (n_hi - n_lo + 1 + 2) as usize;

    let mut f = Vec::new();
    f.resize(m * n, 0);

    for p in dig.keys() {
        let y = (p.0 - m_lo + 1) as usize;
        let x = (p.1 - n_lo + 1) as usize;
        f[y * n + x] = 1;
    }

    let mut queue = Vec::new();
    queue.push((0, 0));

    while !queue.is_empty() {
        let pos = queue.pop().unwrap();
        if f[pos.0 * n + pos.1] != 0 {
            continue;
        }

        f[pos.0 * n + pos.1] = -1;
        for dir in URDL.iter() {
            let mv = (pos.0 as i32 + dir.0, pos.1 as i32 + dir.1);
            if mv.0 < 0 || mv.1 < 0 || mv.0 >= m as i32 || mv.1 >= n as i32 {
                continue;
            }

            queue.push((mv.0 as usize, mv.1 as usize));
        }
    }

    let mut sum = 0;
    for x in f {
        if x >= 0 {
            sum += 1;
        }
    }

    println!("{}", sum);

    // for i in 0..m {
    //     for j in 0..n {
    //         let z = f[i * n + j];
    //         print!(
    //             "{}",
    //             if z == 0 {
    //                 "."
    //             } else if z < 0 {
    //                 " "
    //             } else {
    //                 "#"
    //             }
    //         );
    //     }
    //     println!()
    // }
}
