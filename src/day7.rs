use std::{cmp::Ordering, collections::HashMap};

use crate::util;

const VALUE_ORIG: &str = "23456789TJQKA";
const VALUE_JOKE: &str = "J23456789TQKA";

struct Hand {
    pub code: String,
    pub vals: Vec<i32>,
    pub bets: i32,
    pub kind: i32,
}

impl Hand {
    fn new(code: &str, bet: i32, adv: bool) -> Hand {
        let values: &str = if adv { VALUE_JOKE } else { VALUE_ORIG };

        let v: Vec<i32> = code
            .chars()
            .map(|x| values.find(x).unwrap() as i32)
            .collect();

        let j: Vec<i32> = joke(code)
            .chars()
            .map(|x| values.find(x).unwrap() as i32)
            .collect();

        let k = classify(if adv { &j } else { &v });
        Hand {
            code: code.to_owned(),
            vals: v,
            kind: k,
            bets: bet,
        }
    }
}

fn joke(hand: &str) -> String {
    let mut h = HashMap::new();
    for x in hand.chars().map(|x| VALUE_JOKE.find(x).unwrap() as i32) {
        *h.entry(x).or_insert(0) += 1;
    }

    let ji = VALUE_JOKE.find("J").unwrap() as i32;
    let jc = h.entry(ji).or_insert(0);
    h.remove(&ji);

    let mut ho: Vec<(i32, i32)> = h.into_iter().map(|(x, y)| (-y, x)).collect();
    ho.sort_by(|a, b| a.cmp(b));

    let len = ho.len();
    if len == 0 {
        String::from("JJJJJ")
    } else {
        hand.replace(
            "J",
            &VALUE_JOKE
                .chars()
                .nth(ho[0].1 as usize)
                .unwrap()
                .to_string(),
        )
    }
}

fn classify(hand: &Vec<i32>) -> i32 {
    let mut h = HashMap::new();
    for x in hand {
        *h.entry(x).or_insert(0) += 1;
    }

    let mut ho: Vec<(i32, i32)> = h.into_iter().map(|(x, y)| (-y, *x)).collect();
    ho.sort_by(|a, b| a.cmp(b));

    let len = ho.len();
    if len >= 1 && ho.get(0).unwrap().0 == -5 {
        6
    } else if len >= 1 && ho.get(0).unwrap().0 == -4 {
        5
    } else if len >= 2 && ho.get(0).unwrap().0 == -3 && ho.get(1).unwrap().0 == -2 {
        4
    } else if len >= 2 && ho.get(0).unwrap().0 == -3 {
        3
    } else if len >= 2 && ho.get(0).unwrap().0 == -2 && ho.get(1).unwrap().0 == -2 {
        2
    } else if len >= 2 && ho.get(0).unwrap().0 == -2 {
        1
    } else {
        0
    }
}

fn compare(lhs: &Hand, rhs: &Hand) -> Ordering {
    match lhs.kind.cmp(&rhs.kind) {
        Ordering::Less => Ordering::Less,
        Ordering::Greater => Ordering::Greater,
        Ordering::Equal => {
            for i in 0..5 {
                let c = lhs.vals[i].cmp(&rhs.vals[i]);
                if c != Ordering::Equal {
                    return c;
                }
            }
            Ordering::Equal
        }
    }
}

fn read(filename: &str, adv: bool) -> Vec<Hand> {
    let mut ret: Vec<Hand> = Vec::new();
    if let Ok(content) = util::read_lines(filename) {
        for line in content {
            if let Ok(str) = line {
                let parts: Vec<&str> = str.split(' ').collect();
                ret.push(Hand::new(parts[0], parts[1].parse().unwrap(), adv));
            }
        }
    }

    return ret;
}

pub fn run(filename: &str, adv: bool) {
    let mut cards = read(filename, adv);
    cards.sort_by(|a, b| compare(a, b));
    let mut val = 0;
    for (index, ref hand) in cards.iter().enumerate() {
        val += (index as i32 + 1) * hand.bets;
    }

    println!("{}", val)
}
