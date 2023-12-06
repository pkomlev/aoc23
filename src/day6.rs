use crate::util;
use once_cell::sync::Lazy;
use regex::Regex;

static RE_TEMPLATE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r"(?ms)^Time:(.*)
Distance:(.*)$",
    )
    .unwrap()
});

fn parse_line(line: &str, adv: bool) -> Vec<i64> {
    let mut ret = Vec::new();
    let line = if adv {
        line.replace(" ", "").to_owned()
    } else {
        line.to_owned()
    };
    for part in line.split(' ') {
        if part.is_empty() {
            continue;
        }

        let num: i64 = part.parse().unwrap();
        ret.push(num);
    }
    ret
}

pub fn ways_to_win(time: i64, record: i64) -> u64 {
    let mut ret: u64 = 0;
    for i in 0..(time + 1) {
        let d = i * (time - i);
        if d > record {
            ret += 1
        }
    }

    return ret;
}

pub fn run(filename: &str, adv: bool) {
    if let Ok(content) = util::read_content(filename) {
        let captures = RE_TEMPLATE.captures(&content);
        match captures {
            None => {}
            Some(captures) => {
                let a = parse_line(captures.get(1).map(|x| x.as_str()).unwrap(), adv);
                let b = parse_line(captures.get(2).map(|x| x.as_str()).unwrap(), adv);
                let mut z = 1;
                for x in a.iter().zip(b.iter()) {
                    z *= ways_to_win(*x.0, *x.1);
                }

                println!("{}", z);
            }
        }
    }
}
