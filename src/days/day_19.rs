use itertools::Itertools;
use regex::Regex;

use crate::input::{Input, Part};
use core::hash;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, hash::Hash)]
enum Color {
    R,
    W,
    B,
    U,
    G,
}

pub(super) fn run(input: &Input, part: Part) -> String {
    let mut input_lines = input.get().lines();
    let towel_patterns = input_lines
        .next()
        .unwrap()
        .split(", ")
        .map(|x| {
            x.chars()
                .map(|c| match c {
                    'r' => Color::R,
                    'w' => Color::W,
                    'b' => Color::B,
                    'u' => Color::U,
                    'g' => Color::G,
                    _ => unreachable!(),
                })
                .collect::<Vec<Color>>()
        })
        .collect::<Vec<_>>();
    input_lines.next(); // Skip the empty line

    let tp_str = towel_patterns
        .iter()
        .map(|x| {
            x.iter()
                .map(|c| match c {
                    Color::R => "r",
                    Color::W => "w",
                    Color::B => "b",
                    Color::U => "u",
                    Color::G => "g",
                })
                .collect::<String>()
        })
        .join("|");
    let detect_regex = Regex::new(&format!("^({tp_str})*$")).unwrap();
    println!("Regex: {:?}", detect_regex);

    if part == Part::One {
        let mut valid_designs = 0;
        for (i, design) in input_lines.enumerate() {
            print!("Design {i}: {:?} ", design);
            if detect_regex.is_match(design) {
                valid_designs += 1;
                println!("is valid");
            } else {
                println!("is invalid");
            }
        }
        valid_designs.to_string()
    } else {
        let mut possible_patterns_sum = 0;
        for (i, design) in input_lines.enumerate() {
            print!("Design {i}: {:?} ", design);
            if detect_regex.is_match(design) {
                println!("is valid");
                let design = design
                    .chars()
                    .map(|c| match c {
                        'r' => Color::R,
                        'w' => Color::W,
                        'b' => Color::B,
                        'u' => Color::U,
                        'g' => Color::G,
                        _ => unreachable!(),
                    })
                    .collect::<Vec<Color>>();
                let n = find_pattern_rec(design, &towel_patterns, &mut HashMap::new());
                println!("Num possible patterns: {n}");
                possible_patterns_sum += n;
            } else {
                println!("is invalid");
            }
        }
        possible_patterns_sum.to_string()
    }
}

fn find_pattern_rec(
    design: Vec<Color>,
    towel_patterns: &[Vec<Color>],
    mem: &mut HashMap<Vec<Color>, usize>,
) -> usize {
    if design.is_empty() {
        panic!()
    }
    if let Some(e) = mem.get(&design) {
        return *e;
    }
    let mut num_patterns = 0;
    for pattern in towel_patterns {
        if pattern.len() > design.len() {
            continue;
        }
        if pattern.len() == design.len() {
            if pattern == &design {
                // println!("{:?} == {:?} => 1", pattern, design);
                mem.insert(design.clone(), 1);
                num_patterns += 1;
            }
        } else if pattern == &design[..pattern.len()] {
            let n = find_pattern_rec(design[pattern.len()..].to_vec(), towel_patterns, mem);
            // println!("{:?} in {:?} => {}", pattern, design, n);
            mem.insert(design[pattern.len()..].to_vec(), n);
            num_patterns += n;
        }
    }
    // println!("full design {:?} => {}", design, num_patterns);
    num_patterns
}
