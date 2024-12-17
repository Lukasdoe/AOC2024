use crate::input::{Input, Part};
use itertools::Itertools;
use regex::Regex;

const PRICE_A: u64 = 3;
const PRICE_B: u64 = 1;
const PART_TWO_OFFSET: u64 = 10000000000000;

pub(super) fn run(input: &Input, part: Part) -> String {
    let reg_a = Regex::new(r"Button A: X\+(\d+), Y\+(\d+)").unwrap();
    let reg_b = Regex::new(r"Button B: X\+(\d+), Y\+(\d+)").unwrap();
    let reg_price = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();

    let mut tokens = 0;
    for mut lines in input.get().lines().chunks(4).into_iter() {
        let reg_a_captures = reg_a.captures(lines.next().unwrap()).unwrap();
        let (ax, ay) = (
            reg_a_captures[1].parse::<u64>().unwrap(),
            reg_a_captures[2].parse::<u64>().unwrap(),
        );
        let reg_b_captures = reg_b.captures(lines.next().unwrap()).unwrap();
        let (bx, by) = (
            reg_b_captures[1].parse::<u64>().unwrap(),
            reg_b_captures[2].parse::<u64>().unwrap(),
        );
        let reg_price_captures = reg_price.captures(lines.next().unwrap()).unwrap();
        let (px, py) = if part == Part::One {
            (
                reg_price_captures[1].parse::<u64>().unwrap(),
                reg_price_captures[2].parse::<u64>().unwrap(),
            )
        } else {
            (
                reg_price_captures[1].parse::<u64>().unwrap() + PART_TWO_OFFSET,
                reg_price_captures[2].parse::<u64>().unwrap() + PART_TWO_OFFSET,
            )
        };

        if part == Part::One {
            let mut required_tokens = u64::MAX;
            for a in 0..=py / ay {
                if (py - a * ay) % by != 0 {
                    continue;
                }
                let b = (py - a * ay) / by;
                if a * ax + b * bx != px {
                    continue;
                }
                let objective_value = a * PRICE_A + b * PRICE_B;
                if objective_value < required_tokens {
                    required_tokens = objective_value;
                }
            }
            if required_tokens != u64::MAX {
                tokens += required_tokens;
            }
        } else {
            let a = ax as f64;
            let d = ay as f64;
            let b = bx as f64;
            let e = by as f64;
            let c = px as f64;
            let f = py as f64;

            // use Cramer's rule to solve the system of equations
            // A * x + B * y = C
            // D * x + E * y = F
            //
            // M = |A B|
            //     |D E|
            // Mx = |C B|
            //      |F E|
            // My = |A C|
            //      |D F|
            // x = det(Mx) / det(M)
            // y = det(My) / det(M)

            let det_m = a * e - b * d;
            let det_mx = c * e - b * f;
            let det_my = a * f - c * d;
            let x = (det_mx) / (det_m);
            let y = (det_my) / (det_m);
            // if integer solution exists, add the tokens
            if x.fract() == 0.0 && y.fract() == 0.0 {
                let x = unsafe { x.to_int_unchecked::<u64>() };
                let y = unsafe { y.to_int_unchecked::<u64>() };
                debug_assert_eq!(x * ax + y * bx, px);
                debug_assert_eq!(x * ay + y * by, py);
                tokens += x * PRICE_A + y * PRICE_B;
            }
        }
    }
    tokens.to_string()
}
