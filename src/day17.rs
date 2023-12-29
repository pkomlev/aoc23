use std::collections::{BinaryHeap, HashMap};

use std::cmp::Reverse;

use once_cell::sync::Lazy;

use crate::util;

static URDL: Lazy<Vec<(i32, i32)>> = Lazy::new(|| {
    let mut ret = Vec::new();
    ret.push((-1, 0));
    ret.push((0, 1));
    ret.push((1, 0));
    ret.push((0, -1));
    ret
});

fn node_code(i: i32, j: i32, d: i32) -> i32 {
    i * 256 * 256 + j * 256 + d
}

fn _debug_format_node(c: i32) -> String {
    let n = {
        let d = c % 256;
        let c = c / 256;
        let j = c % 256;
        let i = c / 256;
        (i, j, d)
    };

    format!("{} {} {}", n.0, n.1, n.2)
}

fn create_graph(raw: &Vec<Vec<i32>>, adv: bool) -> HashMap<i32, Vec<(i32, u32)>> {
    let allowed_steps = if adv {
        let ret: Vec<i32> = (4..11).collect();
        ret
    } else {
        let ret: Vec<i32> = (1..4).collect();
        ret
    };

    let mut edges: HashMap<i32, Vec<(i32, u32)>> = HashMap::new();
    for i in 0..raw.len() {
        for j in 0..raw[0].len() {
            for prev in 0..4 {
                for next in 0..4 {
                    if prev % 2 == next % 2 {
                        continue;
                    }

                    for step in &allowed_steps {
                        let step = *step;

                        let ii = (i as i32) + step * URDL[next].0;
                        let jj = (j as i32) + step * URDL[next].1;
                        if ii < 0 || jj < 0 || ii >= raw.len() as i32 || jj >= raw[0].len() as i32 {
                            continue;
                        }

                        let mut cost = 0;
                        for x in 0..step {
                            let xi = (i as i32) + (x + 1) * URDL[next].0;
                            let xj = (j as i32) + (x + 1) * URDL[next].1;
                            cost += raw[xi as usize][xj as usize];
                        }

                        let src = node_code(i as i32, j as i32, prev as i32);
                        let dst = node_code(ii, jj, next as i32);
                        edges
                            .entry(src)
                            .or_insert(Vec::new())
                            .push((dst, cost.try_into().unwrap()));
                    }
                }
            }
        }
    }

    edges
}

fn dijkstra(edges: &HashMap<i32, Vec<(i32, u32)>>, start: &Vec<i32>) -> HashMap<i32, i32> {
    let mut qqq = BinaryHeap::new();
    let mut res = HashMap::new();
    for n in start {
        qqq.push((Reverse(0), *n));
    }

    while let Some((Reverse(d), n)) = qqq.pop() {
        if res.contains_key(&n) {
            continue;
        }

        res.insert(n, d);

        for edge in &edges[&n] {
            if res.contains_key(&edge.0) {
                continue;
            }

            let c = d + (edge.1 as i32);
            qqq.push((Reverse(c), edge.0));
        }
    }

    res
}

pub fn run(filename: &str, adv: bool) {
    let mut raw: Vec<Vec<i32>> = Vec::new();
    if let Ok(lines) = util::read_lines(filename) {
        for (_, line) in lines.enumerate() {
            if let Ok(data) = line {
                raw.push(data.bytes().map(|x| (x - b'0') as i32).collect());
            }
        }
    }

    let edges = create_graph(&raw, adv);
    let dists = dijkstra(&edges, &vec![node_code(0, 0, 0), node_code(0, 0, 1)]);

    let mut min = i32::max_value();
    for i in 0..4 {
        let c = node_code(raw.len() as i32 - 1, raw[0].len() as i32 - 1, i as i32);
        if let Some(value) = dists.get(&c) {
            min = min.min(*value);
        }
    }

    println!("{}", min);
}
