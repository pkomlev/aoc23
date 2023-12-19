use crate::util;
use once_cell::sync::Lazy;
use regex::Regex;

static RE_TEMPLATE: Lazy<Regex> = Lazy::new(|| {
    {
        Regex::new(
            r"(?ms)^seeds: (.*)

seed-to-soil map:
(.*)

soil-to-fertilizer map:
(.*)

fertilizer-to-water map:
(.*)

water-to-light map:
(.*)

light-to-temperature map:
(.*)

temperature-to-humidity map:
(.*)

humidity-to-location map:
(.*)$",
        )
    }
    .unwrap()
});

fn map_seed(change: &Vec<(i64, i64, i64)>, seed: i64) -> i64 {
    for &(dst, src, num) in change {
        if seed >= src && seed < src + num {
            return dst + seed - src;
        }
    }
    seed
}

fn read_seeds(line: &str) -> Vec<(i64, i64)> {
    line.split(' ')
        .map(|f| {
            let v: i64 = f.parse().unwrap();
            (v, v)
        })
        .collect()
}

fn read_map(lines: &str) -> Vec<(i64, i64, i64)> {
    let mut ret = Vec::new();
    for part in lines.split('\n') {
        let part = part.trim();
        if part.is_empty() {
            continue;
        }

        let mapping: Vec<i64> = part
            .split(' ')
            .map(|x| {
                let r: i64 = x.parse().unwrap();
                r
            })
            .collect();

        assert!(mapping.len() == 3);

        ret.push((
            *mapping.get(0).unwrap(),
            *mapping.get(1).unwrap(),
            *mapping.get(2).unwrap(),
        ));
    }

    return ret;
}

pub fn run(filename: &str, adv: bool) {
    if let Ok(content) = util::read_content(filename) {
        let captures = RE_TEMPLATE.captures(&content);
        match captures {
            None => {}
            Some(captures) => {
                let mut seeds = read_seeds(captures.get(1).map(|f| f.as_str()).unwrap());

                let mut changes = Vec::new();
                for i in 2..9 {
                    changes.push(read_map(captures.get(i).map(|f| f.as_str()).unwrap()))
                }

                if !adv {
                    for change in changes {
                        seeds = seeds
                            .iter()
                            .map(|x| (x.0, map_seed(&change, x.1)))
                            .collect();
                    }

                    println!("{}", seeds.iter().map(|x| x.1).min().unwrap());
                } else {
                    let mut curr: i64 = i64::MAX;

                    // NOTE(pkomlev): not proud of this, but that obviously
                    // gives the right answer in just under 10 minutes :)
                    for i in 0..seeds.len() / 2 {
                        let r = seeds.get(2 * i).unwrap().0;
                        let c = seeds.get(2 * i + 1).unwrap().0;

                        for j in 0..c {
                            let mut value = r + j;
                            for change in changes.iter() {
                                value = map_seed(change, value);
                            }

                            curr = i64::min(curr, value);
                        }
                    }

                    println!("{}", curr)
                }
            }
        }
    }
}
