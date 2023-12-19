use std::{
    collections::{HashMap, HashSet},
    mem,
};

use crate::util;
use once_cell::sync::Lazy;

const HAS_U: u32 = 0x1;
const HAS_D: u32 = 0x2;
const HAS_L: u32 = 0x4;
const HAS_R: u32 = 0x8;

static DIR: Lazy<HashMap<u8, u32>> = Lazy::new(|| {
    let mut ret = HashMap::new();
    ret.insert(b'|', HAS_U | HAS_D);
    ret.insert(b'-', HAS_L | HAS_R);
    ret.insert(b'L', HAS_U | HAS_R);
    ret.insert(b'J', HAS_U | HAS_L);
    ret.insert(b'F', HAS_D | HAS_R);
    ret.insert(b'7', HAS_D | HAS_L);
    ret
});

static CON: Lazy<Vec<(i32, i32, u32)>> = Lazy::new(|| {
    let mut ret = Vec::new();
    ret.push((-1, 0, HAS_D));
    ret.push((1, 0, HAS_U));
    ret.push((0, -1, HAS_R));
    ret.push((0, 1, HAS_L));
    return ret;
});

fn find_start(maze: &Vec<Vec<u8>>) -> (i32, i32) {
    for (i, row) in maze.iter().enumerate() {
        for (j, val) in row.iter().enumerate() {
            if *val == b'S' {
                return (i as i32, j as i32);
            }
        }
    }
    panic!()
}

pub fn run(filename: &str, adv: bool) {
    let mut maze: Vec<Vec<u8>> = Vec::new();
    if let Ok(lines) = util::read_lines(filename) {
        for line in lines {
            if let Ok(data) = line {
                maze.push(data.into_bytes());
            }
        }
    }

    let start = find_start(&maze);
    let mut curr = Vec::new();
    curr.push(start.clone());

    let mut visited = HashSet::new();
    visited.insert(start);

    let nrows = maze.len() as i32;
    let ncols = maze[0].len() as i32;

    let mut iter = 0;
    loop {
        iter += 1;
        let mut next = Vec::new();
        for item in &curr {
            for con in CON.iter() {
                let (y, x) = (item.0 + con.0, item.1 + con.1);
                if y < 0 || x < 0 || y >= nrows || x >= ncols {
                    continue;
                }

                let d = maze[y as usize][x as usize];
                let m = DIR.get(&d).unwrap_or(&0);

                if con.2 & m != 0 && !visited.contains(&(y, x)) {
                    next.push((y, x));
                    visited.insert((y, x));
                }
            }
        }

        mem::swap(&mut curr, &mut next);
        if curr.is_empty() {
            break;
        }
    }

    if !adv {
        println!("{}", iter);
    }
}
