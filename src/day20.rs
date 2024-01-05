use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashMap;

use crate::util;

static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(.*) -> (.*)").unwrap());

pub fn run(filename: &str, adv: bool) {
    let raw = HashMap::new();
    if let Ok(lines) = util::read_lines(filename) {
        for (_, line) in lines.enumerate() {
            if let Ok(data) = line {
                if let Some(m) = RE.captures(&data) {
                    let node = m.get(1).unwrap().as_str();
                    let conn = m.get(2).unwrap().as_str();
                    println!("{} {}", node, conn);
                    if node == "broadcaster" {
                        continue;
                    }

                    
                }
            }
        }
    }
}
