use crate::util;
use once_cell::sync::Lazy;
use regex::Regex;

static RE_LINE: Lazy<Regex> = Lazy::new(|| Regex::new(r"Game (\d+): (.*)").unwrap());
static RE_VALUE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\d+) (.*)").unwrap());

fn parse_chunk(chunk: &str) -> (i32, i32, i32) {
    match RE_VALUE.captures(chunk) {
        Some(captures) => {
            let num: i32 = captures
                .get(1)
                .map(|x| x.as_str())
                .unwrap()
                .parse()
                .unwrap();

            let clr = captures.get(2).map(|x| x.as_str()).unwrap();

            if clr == "red" {
                return (num, 0, 0);
            } else if clr == "green" {
                return (0, num, 0);
            } else if clr == "blue" {
                return (0, 0, num);
            }
        }
        None => {}
    }

    (0, 0, 0)
}

fn parse_block(block: &str) -> (i32, i32, i32) {
    let mut r: i32 = 0;
    let mut g: i32 = 0;
    let mut b: i32 = 0;

    let parts = block.split("; ");
    for part in parts.into_iter() {
        let chunks = part.split(", ");
        for chunk in chunks.into_iter() {
            let val = parse_chunk(chunk);
            r = i32::max(r, val.0);
            g = i32::max(g, val.1);
            b = i32::max(b, val.2);
        }
    }

    (r, g, b)
}

pub fn run(filename: &str, adv: bool) {
    let mut code = 0;
    let mut hash = 0;
    if let Ok(lines) = util::read_lines(filename) {
        for line in lines {
            if let Ok(str) = line {
                match RE_LINE.captures(&str) {
                    Some(captures) => {
                        let game: i32 = captures
                            .get(1)
                            .map(|x| x.as_str())
                            .unwrap()
                            .parse()
                            .unwrap();
                        let data = captures.get(2).map(|x| x.as_str()).unwrap();

                        let d = parse_block(data);
                        if d.0 <= 12 && d.1 <= 13 && d.2 <= 14 {
                            code += game;
                        }

                        hash += d.0 * d.1 * d.2;
                    }
                    None => {}
                }
            }
        }
    }

    println!("{}", if adv { hash } else { code });
}
