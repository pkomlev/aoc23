use once_cell::sync::Lazy;
use regex::Regex;

use crate::util;

static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\w) (\d+) \(#(\w+)\)").unwrap());

pub fn run(filename: &str, _: bool) {}
