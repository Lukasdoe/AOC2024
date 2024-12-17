use std::collections::BinaryHeap;

use crate::input::{Input, Part};

pub(super) fn run(input: &Input, part: Part) -> String {
    let re = regex::Regex::new(
        r"Register A: (\d+)
Register B: (\d+)
Register C: (\d+)

Program: ([\d,]+)
",
    )
    .unwrap();
    let captures = re.captures(input.get()).unwrap();
    let a = captures.get(1).unwrap().as_str().parse::<u64>().unwrap();
    let b = captures.get(2).unwrap().as_str().parse::<u64>().unwrap();
    let c = captures.get(3).unwrap().as_str().parse::<u64>().unwrap();
    let program = captures
        .get(4)
        .unwrap()
        .as_str()
        .split(",")
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    if part == Part::One {
        let out = run_program(a, b, c, program, |_| false);
        out.iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(",")
    } else {
        // -2 = 21440491853173
        let mut states = BinaryHeap::new();
        states.push((0, std::cmp::Reverse(0)));
        while let Some((current_match_len, std::cmp::Reverse(a))) = states.pop() {
            println!("current a: {a:#o}");
            for a_offset in 0..=7 {
                let mut i = program.len() - current_match_len - 1;
                let a = (a << 3) | a_offset;
                let mut match_len = 0;

                let _ = run_program(a, b, c, program.clone(), |value| {
                    if i < program.len() && value as usize == program[i] {
                        // continue
                        i += 1;
                        match_len += 1;
                        false
                    } else {
                        // break
                        true
                    }
                });
                if match_len == current_match_len + 1 {
                    if match_len == program.len() {
                        return a.to_string();
                    }
                    states.push((match_len, std::cmp::Reverse(a)));
                }
            }
        }
        panic!();
    }
}

fn run_program<F: FnMut(u64) -> bool>(
    mut a: u64,
    mut b: u64,
    mut c: u64,
    program: Vec<usize>,
    mut output_breaker: F,
) -> Vec<usize> {
    let mut output = Vec::new();
    let mut ip = 0;
    while ip < program.len() {
        let operand = program[ip + 1] as u64;
        let combo_operand = match operand {
            0..=3 => operand,
            4 => a,
            5 => b,
            6 => c,
            7 => panic!("reserved"),
            _ => panic!("invalid operand"),
        };
        // println!(
        //     "ip: {}, opcode: {}, operand: {}, combo_operand: {}, a: {}, b: {}, c: {}",
        //     ip, program[ip], operand, combo_operand, a, b, c
        // );
        match program[ip] {
            0 => {
                let numerator = a;
                let denominator = 2_u64.pow(combo_operand as u32);
                a = numerator / denominator;
                ip += 2;
            }
            1 => {
                b ^= operand;
                ip += 2;
            }
            2 => {
                b = combo_operand % 8;
                ip += 2;
            }
            3 => {
                if a != 0 {
                    ip = operand as usize;
                } else {
                    ip += 2;
                }
            }
            4 => {
                b ^= c;
                ip += 2;
            }
            5 => {
                let value = combo_operand % 8;
                output.push(value as usize);
                // println!("{}", value);
                if output_breaker(value) {
                    break;
                }
                ip += 2;
            }
            6 => {
                let numerator = a;
                let denominator = 2_u64.pow(combo_operand as u32);
                b = numerator / denominator;
                ip += 2;
            }
            7 => {
                let numerator = a;
                let denominator = 2_u64.pow(combo_operand as u32);
                c = numerator / denominator;
                ip += 2;
            }
            _ => panic!("invalid opcode"),
        }
    }
    output
}
