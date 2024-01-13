use once_cell::sync::Lazy;

use crate::util;

static URDL: Lazy<Vec<(isize, isize)>> = Lazy::new(|| vec![(-1, 0), (0, 1), (1, 0), (0, -1)]);

struct Neighbors {
    y: usize,
    x: usize,
    maxy: usize,
    maxx: usize,
    curr: usize,
}

impl Iterator for Neighbors {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        while self.curr < URDL.len() {
            let yy = self.y as isize + URDL[self.curr].0;
            let xx = self.x as isize + URDL[self.curr].1;
            self.curr += 1;

            if yy >= 0 && xx >= 0 && (yy as usize) < self.maxy && (xx as usize) < self.maxx {
                return Some((yy as usize, xx as usize));
            }
        }

        None
    }
}

fn neighbors(y: usize, x: usize, maxy: usize, maxx: usize) -> Neighbors {
    Neighbors {
        y,
        x,
        maxy,
        maxx,
        curr: 0,
    }
}

fn find_hole(d: &Vec<u8>) -> usize {
    d.iter().enumerate().find(|x| *x.1 == b'.').unwrap().0
}

pub fn run(filename: &str, adv: bool) {
    if adv {
        return;
    }

    let mut input: Vec<Vec<u8>> = Vec::new();
    if let Ok(lines) = util::read_lines(filename) {
        for line in lines {
            if let Ok(line) = line {
                input.push(line.bytes().collect());
            }
        }
    }

    let start = (0_usize, find_hole(&input[0]));
    let end = (input.len() - 1, find_hole(&input[input.len() - 1]));

    let nrow = input.len();
    let ncol = input[0].len();

    for i in 0..nrow {
        for j in 0..ncol {
            if input[i][j] == b'#' {
                continue;
            }

            let mut space = 0;
            let mut dir = 0;

            for (ni, nj) in neighbors(i, j, nrow, ncol) {
                let c = input[ni][nj];
                if c == b'.' {
                    space += 1;
                    continue;
                }

                if c == b'<' || c == b'>' || c == b'v' || c == b'^' {
                    dir += 1;
                    continue;
                }
            }

            // if dir >= 1 && space > 1 {
            //     println!("here 1");
            // }

            // if dir == 0 && space > 2 {
            //     println!("here {} {}", i, j);
            // }
        }
    }
}
