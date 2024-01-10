use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::{HashMap, VecDeque};

use crate::util;

static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(.*) -> (.*)").unwrap());

struct Problem {
    names: HashMap<String, usize>,
    nodes: Vec<(bool, Vec<usize>, u64)>,
    broad: Vec<usize>,
}

fn parse_conn(line: &str) -> Vec<String> {
    line.split(',').map(|x| x.trim().to_owned()).collect()
}

fn parse_task(filename: &str) -> Problem {
    let mut nodules: HashMap<String, (bool, Vec<String>)> = HashMap::new();
    let mut broadcast = Vec::new();

    if let Ok(lines) = util::read_lines(filename) {
        for (_, line) in lines.enumerate() {
            if let Ok(data) = line {
                if let Some(m) = RE.captures(&data) {
                    let node = m.get(1).unwrap().as_str();
                    let conn = m.get(2).unwrap().as_str();

                    let conn = parse_conn(conn);
                    if node == "broadcaster" {
                        broadcast.extend(conn);
                        continue;
                    }

                    if node.chars().nth(0) == Some('%') {
                        nodules.insert(node.split_at(1).1.to_owned(), (false, conn));
                    } else {
                        nodules.insert(node.split_at(1).1.to_owned(), (true, conn));
                    }
                }
            }
        }
    }

    let mut index = HashMap::new();
    let mut up = |name: &str| {
        if !index.contains_key(name) {
            let id = index.len();
            index.insert(name.to_owned(), id);
        }
    };

    for (this, data) in &nodules {
        up(this);
        for next in &data.1 {
            up(&next);
        }
    }

    let mut nodes = Vec::new();
    nodes.resize(index.len(), (false, Vec::new(), 0));
    for (nodule, data) in &nodules {
        let this = *index.get(nodule).unwrap();
        nodes[this].0 = data.0;

        let mask = 1_u64 << this;
        for next in &data.1 {
            let next = *index.get(next).unwrap();
            nodes[next].2 |= mask;
            nodes[this].1.push(next);
        }
    }

    let broad = broadcast.iter().map(|x| *index.get(x).unwrap()).collect();

    Problem {
        names: index,
        nodes,
        broad,
    }
}

pub fn run(filename: &str, adv: bool) {
    let task = parse_task(filename);

    // notation:
    // - 0 is for the low pulse.
    // - 0 for % module - off state
    // - mask for & module - memory
    let mut state = Vec::new();
    state.resize(task.nodes.len(), 0_u64);

    let mut hi = 0_i64;
    let mut lo = 0_i64;

    let mut queue = VecDeque::new();

    let machine = |queue: &mut VecDeque<(usize, usize, bool)>,
                   state: &mut Vec<u64>,
                   from: usize,
                   to: usize,
                   is_high: bool| {
        let nodule: &(bool, Vec<usize>, u64) = &task.nodes[to];
        if nodule.0 {
            // .. conjunction
            let mask = 1 << from;
            if is_high {
                state[to] |= mask;
            } else {
                state[to] &= !mask;
            }

            for n in &nodule.1 {
                queue.push_back((to, *n, state[to] != nodule.2));
            }
        } else {
            // .. flip flop
            if !is_high {
                state[to] = !state[to];
                for n in &nodule.1 {
                    queue.push_back((to, *n, state[to] != 0));
                }
            }
        }
    };

    if !adv {
        for _ in 0..1000 {
            lo += 1;

            for bc in &task.broad {
                queue.push_back((64_usize, *bc, false));
            }

            while !queue.is_empty() {
                let (from, to, is_high) = queue.pop_front().unwrap();
                if is_high {
                    hi += 1;
                } else {
                    lo += 1;
                }

                machine(&mut queue, &mut state, from, to, is_high);
            }
        }

        println!("{}", hi * lo);
    } else {
        let acc = task.names["dh"];

        for iter in 0..100_000 {
            for bc in &task.broad {
                queue.push_back((64_usize, *bc, false));
            }
            while !queue.is_empty() {
                let (from, to, is_high) = queue.pop_front().unwrap();
                let before = state[acc];
                machine(&mut queue, &mut state, from, to, is_high);
                if before != state[acc] {
                    println!("{} - {:b} {:b}", iter, before, state[acc]);
                }
            }
        }

        // TODO(pkomlev): make this a little bit more repeatable.
    }
}
