use std::collections::{HashMap, HashSet};

use crate::util;
use once_cell::sync::Lazy;
use regex::Regex;

static RE_LINE: Lazy<Regex> = Lazy::new(|| Regex::new(r"Card\s+(\d+): (.*) \| (.*)").unwrap());

fn parse_card(line: &str) -> HashSet<i32> {
    let mut card = HashSet::new();
    for part in line.split(' ') {
        if part.is_empty() {
            continue;
        }

        let num: i32 = part.parse().unwrap();
        card.insert(num);
    }
    card
}

pub fn run(filename: &str, adv: bool) {
    let mut result = 0;
    let mut mulcnt: i32 = 0;

    let mut reward: HashMap<i32, i32> = HashMap::new();

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

                        let win = parse_card(captures.get(2).unwrap().as_str());
                        let num = parse_card(captures.get(3).unwrap().as_str());

                        let mut count: i32 = 0;
                        for n in num.iter() {
                            if win.contains(n) {
                                count += 1;
                            }
                        }

                        let this_count = reward.get(&game).unwrap_or(&0) + 1;
                        if count > 0 {
                            result += 2_i32.pow((count - 1) as u32);
                            for i in 0..count {
                                let old_count = reward.get(&(game + i + 1)).unwrap_or(&0);
                                reward.insert(game + i + 1, this_count + old_count);
                            }
                        }

                        mulcnt += this_count;
                    }
                    None => {}
                }
            }
        }
    }

    println!("{}", if adv { mulcnt } else { result });
}
