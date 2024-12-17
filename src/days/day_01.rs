use std::collections::HashMap;

use crate::input::{Input, Part};

pub(super) fn run(input: &Input, part: Part) -> String {
    let mut left: Vec<u64> = Vec::new();
    let mut right: Vec<u64> = Vec::new();
    for line in input.get().lines() {
        let (l, r) = line.split_once("   ").unwrap();
        left.push(l.parse().unwrap());
        right.push(r.parse().unwrap());
    }
    if part == Part::One {
        left.sort();
        right.sort();
        let mut sum = 0;
        for i in 0..left.len() {
            sum += left[i].abs_diff(right[i]);
        }
        sum.to_string()
    } else {
        let mut occur = HashMap::new();
        for obj in right {
            occur.entry(obj).and_modify(|e| *e += 1).or_insert(1);
        }
        let mut res = 0;
        for obj in left {
            if let Some(e) = occur.get_mut(&obj) {
                res += obj * *e;
            }
        }
        res.to_string()
    }
}
