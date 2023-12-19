use std::{
    collections::{HashMap, HashSet},
    mem,
};

use crate::util;
use once_cell::sync::Lazy;
use regex::Regex;

static RE_TEMPLATE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r"(?ms)^([RL]*)

(.*)$",
    )
    .unwrap()
});

static RE_NODE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\w+) = \((\w+), (\w+)\)").unwrap());

struct Node {
    left: String,
    right: String,
}

fn run_simple(graph: &HashMap<String, Node>, direction: &Vec<char>) {
    let mut node: &str = "AAA";
    let mut iter = 0;
    loop {
        if direction[iter % direction.len()] == 'L' {
            node = &graph.get(node).unwrap().left;
        } else {
            node = &graph.get(node).unwrap().right;
        }

        iter += 1;

        if node == "ZZZ" {
            break;
        }
    }

    println!("{}", iter);
}

fn run_adv(graph: &HashMap<String, Node>, direction: &Vec<char>) {
    let mut curr: Vec<&str> = Vec::new();
    let mut next: Vec<&str> = Vec::new();
    for (k, _) in graph.iter() {
        if k.ends_with('A') {
            curr.push(&k);
        }
    }

    let mut matches: Vec<Vec<usize>> = Vec::new();
    matches.resize(curr.len(), Vec::new());

    let mut iter = 0;
    loop {
        next.clear();

        for (idx, node) in curr.iter().enumerate() {
            if direction[iter % direction.len()] == 'L' {
                next.push(&graph.get(*node).unwrap().left);
            } else {
                next.push(&graph.get(*node).unwrap().right);
            }
        }

        iter += 1;

        let mut count = 0;
        for (idx, node) in next.iter().enumerate() {
            if node.ends_with('Z') {
                matches[idx].push(iter);
                count += 1;
            }
        }

        if count >= 1 {
            println!("------");
            for (idx, positions) in matches.iter().enumerate() {
                if positions.len() >= 2 {
                    println!(
                        "{} -- delta {} div {}",
                        idx,
                        positions[positions.len() - 1] - positions[positions.len() - 2],
                        positions[positions.len() - 1] % positions[0]
                    );
                }
            }
        }

        mem::swap(&mut curr, &mut next);
    }

    // TODO(pkomlev): i kind of figured at this point that the following is the cycle: 
    // 0 -- delta 20803 div 0
    // 1 -- delta 23147 div 0
    // 2 -- delta 17873 div 0
    // 3 -- delta 17287 div 0
    // 4 -- delta 13771 div 0
    // 5 -- delta 19631 div 0
    // -- lcm is 18625484023687
}

pub fn run(filename: &str, adv: bool) {
    let mut direction: String = String::new();
    let mut graph: HashMap<String, Node> = HashMap::new();

    if let Ok(content) = util::read_content(filename) {
        let captures = RE_TEMPLATE.captures(&content);
        match captures {
            None => {
                panic!();
            }
            Some(captures) => {
                direction = captures.get(1).map(|x| x.as_str()).unwrap().to_owned();
                let nodes = captures.get(2).map(|x| x.as_str()).unwrap();

                for node in nodes.split('\n') {
                    let captures = RE_NODE.captures(node);
                    match captures {
                        None => {
                            panic!();
                        }
                        Some(captures) => {
                            let x = captures.get(1).map(|x| x.as_str()).unwrap();
                            let y = captures.get(2).map(|x| x.as_str()).unwrap();
                            let z = captures.get(3).map(|x| x.as_str()).unwrap();

                            graph.insert(
                                x.to_owned(),
                                Node {
                                    left: y.to_owned(),
                                    right: z.to_owned(),
                                },
                            );
                        }
                    }
                }
            }
        }
    }

    let direction: Vec<char> = direction.chars().collect();

    if adv {
        run_adv(&graph, &direction)
    } else {
        run_simple(&graph, &direction)
    }
}
