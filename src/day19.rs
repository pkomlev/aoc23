use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashMap;

use crate::util;

static M: Lazy<HashMap<&str, usize>> = Lazy::new(|| {
    let mut ret = HashMap::new();
    ret.insert(",", ret.len());
    ret.insert("x", ret.len());
    ret.insert("m", ret.len());
    ret.insert("a", ret.len());
    ret.insert("s", ret.len());
    ret
});

struct Rule {
    idx: usize,
    gt: bool,
    val: i32,
    dst: String,
}

static RE_INPUT_1: Lazy<Regex> = Lazy::new(|| Regex::new(r"\{(.*)\}").unwrap());
static RE_INPUT_2: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\w)=(\d+)").unwrap());
static RE_RULE_1: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\w+)\{(.*)\}").unwrap());
static RE_RULE_2: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\w)([<>])(\d+):(\w+)").unwrap());

fn parse_rule(repr: &str) -> Rule {
    match RE_RULE_2.captures(repr) {
        Some(captures) => Rule {
            idx: M[captures.get(1).unwrap().as_str()],
            gt: captures.get(2).unwrap().as_str() == ">",
            val: captures.get(3).unwrap().as_str().parse().unwrap(),
            dst: captures.get(4).unwrap().as_str().to_owned(),
        },
        None => Rule {
            idx: 0,
            gt: true,
            val: i32::min_value(),
            dst: repr.to_owned(),
        },
    }
}

fn rule_matches(rule: &Rule, input: &Vec<i32>) -> bool {
    let val = input[rule.idx];
    if rule.gt {
        val > rule.val
    } else {
        val < rule.val
    }
}

pub fn run(filename: &str, _: bool) {
    let mut new_line = false;
    let mut rules: HashMap<String, Vec<Rule>> = HashMap::new();
    let mut inputs: Vec<Vec<i32>> = Vec::new();
    if let Ok(lines) = util::read_lines(filename) {
        for (_, line) in lines.enumerate() {
            if let Ok(data) = line {
                if data.is_empty() {
                    new_line = true;
                }

                if new_line {
                    if let Some(matches) = RE_INPUT_1.captures(&data) {
                        let values = matches.get(1).unwrap().as_str();
                        let mut attr = Vec::new();
                        attr.resize(M.len(), 0);
                        for token in values.split(',') {
                            if let Some(matches) = RE_INPUT_2.captures(token) {
                                attr[M[matches.get(1).unwrap().as_str()]] =
                                    matches.get(2).unwrap().as_str().parse().unwrap();
                            }
                        }
                        inputs.push(attr);
                    }
                } else {
                    if let Some(matches) = RE_RULE_1.captures(&data) {
                        let node = matches.get(1).map(|x| x.as_str()).unwrap();
                        let workflow = matches
                            .get(2)
                            .map(|x: regex::Match<'_>| x.as_str())
                            .unwrap();

                        let mut ruleset = Vec::new();
                        for repr in workflow.split(',') {
                            let rule = parse_rule(repr);
                            ruleset.push(rule);
                        }

                        rules.insert(node.to_owned(), ruleset);
                    }
                }
            }
        }
    }

    let mut ret = 0;
    for input in inputs {
        let mut curr = "in";
        while curr != "A" && curr != "R" {
            let ruleset = &rules[curr];
            for rule in ruleset {
                if rule_matches(&rule, &input) {
                    curr = &rule.dst;
                    break;
                }
            }
        }

        if curr == "A" {
            ret += input.iter().sum::<i32>();
        }
    }

    println!("{}", ret);
}
