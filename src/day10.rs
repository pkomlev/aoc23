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
    let mut ret: HashMap<u8, u32> = HashMap::new();
    ret.insert(b'|', HAS_U | HAS_D);
    ret.insert(b'-', HAS_L | HAS_R);
    ret.insert(b'L', HAS_U | HAS_R);
    ret.insert(b'J', HAS_U | HAS_L);
    ret.insert(b'F', HAS_D | HAS_R);
    ret.insert(b'7', HAS_D | HAS_L);
    ret.insert(b'S', HAS_D | HAS_U | HAS_L | HAS_R);
    ret
});

static CON: Lazy<Vec<(i32, i32, u32, u32)>> = Lazy::new(|| {
    let mut ret = Vec::new();
    ret.push((-1, 0, HAS_U, HAS_D));
    ret.push((1, 0, HAS_D, HAS_U));
    ret.push((0, -1, HAS_L, HAS_R));
    ret.push((0, 1, HAS_R, HAS_L));
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

fn find_route(maze: &Vec<Vec<u8>>, start: (i32, i32)) -> HashSet<(i32, i32)> {
    let mut curr = Vec::new();
    curr.push(start.clone());

    let mut visited = HashSet::new();
    visited.insert(start);

    let nrows = maze.len() as i32;
    let ncols = maze[0].len() as i32;

    loop {
        let mut next = Vec::new();
        for item in &curr {
            for con in CON.iter() {
                let d = maze[item.0 as usize][item.1 as usize];
                let m = DIR.get(&d).unwrap_or(&0);
                if con.2 & m == 0 {
                    continue;
                }

                let (y, x) = (item.0 + con.0, item.1 + con.1);
                if y < 0 || x < 0 || y >= nrows || x >= ncols {
                    continue;
                }

                let d = maze[y as usize][x as usize];
                let m = DIR.get(&d).unwrap_or(&0);

                if con.3 & m != 0 && !visited.contains(&(y, x)) {
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

    visited
}

fn rescale(maze: &Vec<Vec<u8>>, visited: &HashSet<(i32, i32)>) -> Vec<Vec<u8>> {
    let nrows = maze.len();
    let ncols = maze[0].len();

    let mut x2: Vec<Vec<u8>> = Vec::new();
    x2.resize(nrows as usize * 2 + 1, Vec::new());

    for i in 0..x2.len() {
        let mut r = Vec::new();
        r.resize(ncols as usize * 2 + 1, b'.');
        x2[i] = r;
    }

    let dir: Vec<(u32, usize, usize)> =
        vec![(HAS_U, 0, 1), (HAS_D, 2, 1), (HAS_L, 1, 0), (HAS_R, 1, 2)];

    for i in 0..nrows {
        for j in 0..ncols {
            if !visited.contains(&(i as i32, j as i32)) {
                continue;
            }

            let m = DIR.get(&maze[i][j]).unwrap();

            x2[2 * i + 1][2 * j + 1] = b'#';

            for d in &dir {
                if m & d.0 != 0 {
                    x2[2 * i + d.1][2 * j + d.2] = b'#';
                }
            }
        }
    }

    x2
}

fn flood(maze: &mut Vec<Vec<u8>>) -> i32 {
    let mut curr = Vec::new();
    curr.push((0_usize, 0_usize));

    let mut next: Vec<(usize, usize)> = Vec::new();

    while !curr.is_empty() {
        next.clear();

        for &(y, x) in &curr {
            if maze[y][x] != b'.' {
                continue;
            }

            maze[y][x] = b'#';
            for d in CON.iter() {
                if y as i32 + d.0 < 0
                    || y as i32 + d.0 >= maze.len() as i32
                    || x as i32 + d.1 < 0
                    || x as i32 + d.1 >= maze[0].len() as i32
                {
                    continue;
                }

                next.push(((y as i32 + d.0) as usize, (x as i32 + d.1) as usize));
            }
        }

        mem::swap(&mut curr, &mut next);
    }

    let mut count = 0;
    for i in 0..maze.len() {
        for j in 0..maze[0].len() {
            if i % 2 == 1 && j % 2 == 1 && maze[i][j] == b'.' {
                count += 1;
            }
        }
    }

    count
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
    let route = find_route(&maze, start);
    if !adv {
        println!("{}", route.len() / 2);
        return;
    }

    let mut x2 = rescale(&maze, &route);
    let val = flood(&mut x2);
    println!("{}", val);
}
