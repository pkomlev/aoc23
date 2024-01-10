use std::{
    collections::{HashMap, HashSet},
    mem,
};

use crate::util;

#[derive(Clone, Copy)]
enum Dir {
    X = 0_isize,
    Y = 1_isize,
    Z = 2_isize,
}

#[derive(Clone, Copy)]
struct Block {
    beg: (i32, i32, i32),
    dir: Dir,
    len: usize,
}

struct BlockIterator<'a> {
    block: &'a Block,
    index: usize,
}

impl<'a> Iterator for BlockIterator<'a> {
    type Item = (i32, i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.block.len {
            let result = match self.block.dir {
                Dir::X => (
                    self.block.beg.0 + self.index as i32,
                    self.block.beg.1,
                    self.block.beg.2,
                ),
                Dir::Y => (
                    self.block.beg.0,
                    self.block.beg.1 + self.index as i32,
                    self.block.beg.2,
                ),
                Dir::Z => (
                    self.block.beg.0,
                    self.block.beg.1,
                    self.block.beg.2 + self.index as i32,
                ),
            };

            self.index += 1;

            Some(result)
        } else {
            None
        }
    }
}

impl Block {
    fn iter(&self) -> BlockIterator {
        BlockIterator {
            block: &self,
            index: 0,
        }
    }

    fn rebase(&self, s: (i32, i32, i32)) -> Block {
        Block {
            beg: s,
            dir: self.dir,
            len: self.len,
        }
    }

    fn new(mut s: (i32, i32, i32), mut e: (i32, i32, i32)) -> Block {
        if s.0 > e.0 || s.1 > e.1 || s.2 > e.2 {
            mem::swap(&mut s, &mut e);
        }

        let d = (e.0 - s.0, e.1 - s.1, e.2 - s.2);

        let mut dir = Dir::X;
        let mut len = 0;

        if d.0 != 0 {
            dir = Dir::X;
            len = d.0;
        }

        if d.1 != 0 {
            dir = Dir::Y;
            len = d.1;
        }

        if d.2 != 0 {
            dir = Dir::Z;
            len = d.2;
        }

        assert!(len >= 0);
        assert!(len == d.0 + d.1 + d.2);

        Block {
            beg: s,
            dir: dir,
            len: len as usize + 1,
        }
    }
}

fn solve_pt1(supports: &Vec<HashSet<usize>>, standson: &Vec<HashSet<usize>>) -> i64 {
    let mut count = 0;
    for list in supports {
        let mut safe = true;
        for brick in list {
            if standson[*brick].len() == 1 {
                safe = false;
            }
        }

        if safe {
            count += 1;
        }
    }

    count
}

fn solve_pt2(_: &Vec<HashSet<usize>>, standson: &Vec<HashSet<usize>>) -> i64 {
    let mut count = 0;
    for remove in 0..standson.len() {
        let mut removed = HashSet::new();
        removed.insert(remove);

        for (brick, deps) in standson.iter().enumerate() {
            if deps.len() > 0 && deps.iter().all(|x| removed.contains(x)) {
                removed.insert(brick);
            }
        }

        count += removed.len() - 1;
    }

    count as i64
}

pub fn run(filename: &str, adv: bool) {
    let parse = |x: &str| -> (i32, i32, i32) {
        let parts: Vec<&str> = x.split(',').into_iter().collect();
        assert_eq!(parts.len(), 3);
        (
            parts[0].parse().unwrap(),
            parts[1].parse().unwrap(),
            parts[2].parse().unwrap(),
        )
    };

    let mut input = Vec::new();
    if let Ok(lines) = util::read_lines(filename) {
        for line in lines {
            if let Ok(line) = line {
                let (a, b) = line.split_once('~').unwrap();
                input.push((parse(a), parse(b)));
            }
        }
    }

    let mut blocks = Vec::new();
    for (s, e) in &input {
        let block = Block::new(s.clone(), e.clone());
        blocks.push(block);
    }

    blocks.sort_by_key(|x| x.beg.2);

    let mut landscape: HashMap<(i32, i32), (usize, i32)> = HashMap::new();
    let mut standson = Vec::new();
    let mut supports = Vec::new();
    supports.resize(blocks.len(), HashSet::new());

    for (index, block) in blocks.iter().enumerate() {
        let mut max = -1;
        for pt in block.iter() {
            if let Some((_, val)) = landscape.get(&(pt.0, pt.1)) {
                max = max.max(*val);
            }
        }

        let mut supp = HashSet::new();
        for pt in block.iter() {
            if let Some((other, val)) = landscape.get(&(pt.0, pt.1)) {
                if *val == max {
                    supp.insert(*other);
                }
            }
        }

        for i in &supp {
            supports[*i].insert(index);
        }

        standson.push(supp);

        max += 1;

        let block = block.rebase((block.beg.0, block.beg.1, max));
        for pt in block.iter() {
            landscape.insert((pt.0, pt.1), (index, pt.2));
        }
    }

    let result = if !adv {
        solve_pt1(&supports, &standson)
    } else {
        solve_pt2(&supports, &standson)
    };

    println!("{}", result);
}
