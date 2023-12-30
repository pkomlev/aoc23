use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashMap;

use crate::util;

static M: Lazy<HashMap<&str, usize>> = Lazy::new(|| {
    let mut ret = HashMap::new();
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

struct RulesSet {
    rules: Vec<Rule>,
    fallback: String,
}

static RE_INPUT_1: Lazy<Regex> = Lazy::new(|| Regex::new(r"\{(.*)\}").unwrap());
static RE_INPUT_2: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\w)=(\d+)").unwrap());
static RE_RULE_1: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\w+)\{(.*)\}").unwrap());
static RE_RULE_2: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\w)([<>])(\d+):(\w+)").unwrap());

fn parse_rule(repr: &str, ruleset: &mut RulesSet) {
    match RE_RULE_2.captures(repr) {
        Some(captures) => ruleset.rules.push(Rule {
            idx: M[captures.get(1).unwrap().as_str()],
            gt: captures.get(2).unwrap().as_str() == ">",
            val: captures.get(3).unwrap().as_str().parse().unwrap(),
            dst: captures.get(4).unwrap().as_str().to_owned(),
        }),
        None => ruleset.fallback = repr.to_owned(),
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

fn count(rules: &HashMap<String, RulesSet>, curr: &str, bounds: &Vec<(i32, i32)>) -> i64 {
    if curr == "R" {
        return 0;
    }

    if curr == "A" {
        let mut mul: i64 = 1;
        for range in bounds {
            let d: i64 = (range.1 - range.0 + 1).into();
            mul *= d;
        }

        return mul;
    }

    let mut work = bounds.clone();
    let mut sum = 0;

    let ruleset = &rules[curr];
    for rule in &ruleset.rules {
        // case when rule matches.
        let saved = work[rule.idx];
        if rule.gt {
            let lower = (rule.val + 1).max(work[rule.idx].0);
            if lower <= work[rule.idx].1 {
                work[rule.idx].0 = lower;
                sum += count(rules, &rule.dst, &work);
            }
        } else {
            let upper = (rule.val - 1).min(work[rule.idx].1);
            if upper >= work[rule.idx].0 {
                work[rule.idx].1 = upper;
                sum += count(rules, &rule.dst, &work);
            }
        }

        // negate and continue.
        work[rule.idx] = saved;
        if rule.gt {
            work[rule.idx].1 = rule.val;
        } else {
            work[rule.idx].0 = rule.val;
        }

        // if interval becomes invalid - done here.
        if work[rule.idx].0 > work[rule.idx].1 {
            return sum;
        }
    }

    sum + count(rules, &ruleset.fallback, &work)
}

pub fn run(filename: &str, adv: bool) {
    let mut new_line = false;
    let mut rules: HashMap<String, RulesSet> = HashMap::new();
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

                        let mut ruleset = RulesSet {
                            rules: Vec::new(),
                            fallback: "R".to_owned(),
                        };
                        for repr in workflow.split(',') {
                            parse_rule(repr, &mut ruleset);
                        }

                        rules.insert(node.to_owned(), ruleset);
                    }
                }
            }
        }
    }

    if adv {
        let mut bounds = Vec::new();
        bounds.resize(M.len(), (1, 4000));
        let sum = count(&rules, "in", &bounds);
        println!("{}", sum);
    } else {
        let mut ret = 0;
        for input in inputs {
            let mut curr = "in";
            while curr != "A" && curr != "R" {
                let ruleset = &rules[curr];
                let prev = curr;
                for rule in &ruleset.rules {
                    if rule_matches(&rule, &input) {
                        curr = &rule.dst;
                        break;
                    }
                }

                if prev == curr {
                    curr = &ruleset.fallback;
                }
            }

            if curr == "A" {
                ret += input.iter().sum::<i32>();
            }
        }

        println!("{}", ret);
    }
}
