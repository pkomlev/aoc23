use std::{collections::HashSet, mem};

use crate::util;

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

pub fn run(filename: &str, _: bool) {
    let mut maze: Vec<Vec<u8>> = Vec::new();
    if let Ok(lines) = util::read_lines(filename) {
        for line in lines {
            if let Ok(line) = line {
                maze.push(line.as_bytes().iter().map(|x| x.clone()).collect());
            }
        }
    }

    let start = find_start(&maze);

    let mut curr = HashSet::new();
    curr.insert(start);

    let mut next = HashSet::new();

    let dir = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
    for _ in 0..64 {
        next.clear();
        for p in &curr {
            for d in &dir {
                let i = p.0 + d.0;
                let j = p.1 + d.1;
                if i < 0
                    || j < 0
                    || i >= maze.len() as i32
                    || i >= maze[0].len() as i32
                    || maze[i as usize][j as usize] == b'#'
                {
                    continue;
                }

                next.insert((i, j));
            }
        }

        mem::swap(&mut curr, &mut next);
    }

    println!("{}", curr.len());
}
