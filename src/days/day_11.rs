use std::collections::HashMap;

use crate::input::{Input, Part};

const PART_TWO_DEPTH: usize = 75;

pub(super) fn run(input: &Input, part: Part) -> u64 {
    let mut stones = input
        .get()
        .lines()
        .next()
        .unwrap()
        .split(" ")
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    if part == Part::One {
        for _ in 0..25 {
            for i in 0..stones.len() {
                if stones[i] == 0 {
                    stones[i] = 1;
                } else {
                    let num_digits = ((stones[i] as f32).log10().floor() + 1.0) as u32;
                    if num_digits % 2 == 0 {
                        let powten = 10_u64.pow(num_digits / 2);
                        stones.push(stones[i] / powten);
                        stones[i] %= powten;
                    } else {
                        stones[i] *= 2024;
                    }
                }
            }
        }
        stones.len() as u64
    } else {
        let mut mem = HashMap::new();
        let res = stones.iter().map(|&s| stones_rec(0, s, &mut mem)).sum();
        res
    }
}

fn stones_rec(mut depth: usize, mut stone: u64, mem: &mut HashMap<(u64, usize), u64>) -> u64 {
    loop {
        if depth == PART_TWO_DEPTH {
            return 1;
        }
        depth += 1;
        if mem.contains_key(&(stone, depth)) {
            return mem[&(stone, depth)];
        }

        if stone == 0 {
            stone = 1;
        } else {
            let num_digits = stone.ilog10() + 1;
            if num_digits % 2 == 0 {
                let powten = 10_u64.pow(num_digits / 2);
                let res =
                    stones_rec(depth, stone / powten, mem) + stones_rec(depth, stone % powten, mem);
                mem.insert((stone, depth), res);
                return res;
            } else {
                stone *= 2024;
            }
        }
    }
}
