use crate::util;
use once_cell::sync::Lazy;

static LITERALS_S0: Lazy<Vec<(String, u32)>> = Lazy::new(|| {
    let mut ret: Vec<(String, u32)> = Vec::new();
    for i in 0..10 {
        ret.push((i.to_string(), i));
    }
    ret
});

static LITERALS_S1: Lazy<Vec<(String, u32)>> = Lazy::new(|| {
    let mut ret: Vec<(String, u32)> = Vec::new();
    for (n, i) in LITERALS_S0.iter() {
        ret.push((n.clone(), *i));
    }

    let named: Vec<(&str, u32)> = Vec::from([
        ("zero", 0),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    for (n, i) in named {
        ret.push((n.to_owned(), i));
    }
    ret
});

fn calibration(line: String, vocab: &Vec<(String, u32)>) -> u32 {
    let mut first: (u32, i32) = (0, i32::MAX);
    let mut last: (u32, i32) = (0, i32::MIN);

    for (s, v) in vocab {
        match line.find(s) {
            Some(pos) => {
                if first.1 > pos as i32 {
                    first = (*v, pos as i32);
                }
            }
            None => {}
        }
        match line.rfind(s) {
            Some(pos) => {
                if last.1 < pos as i32 {
                    last = (*v, pos as i32);
                }
            }
            None => {}
        }
    }

    first.0 * 10 + last.0
}

pub fn run(filename: &str, adv: bool) {
    let vocab = if adv { &LITERALS_S1 } else { &LITERALS_S0 };
    if let Ok(lines) = util::read_lines(filename) {
        let mut value: u32 = 0;
        for line in lines {
            if let Ok(str) = line {
                value += calibration(str, &vocab);
            }
        }

        println!("{}", value);
    }
}
