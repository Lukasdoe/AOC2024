use crate::input::{Input, Part};
use itertools::Itertools;
use std::{collections::HashMap, ops::AddAssign};

const NUM_NEW_RANDS: usize = 2000;
// const NUM_NEW_RANDS: usize = 10;

pub(super) fn run(input: &Input, part: Part) -> String {
    if part == Part::One {
        let mut running_sum = 0;
        for line in input.get().lines() {
            let init = line.parse::<u64>().unwrap();
            let mut secret = init;
            for _ in 0..NUM_NEW_RANDS {
                secret = next_secret(secret);
            }
            running_sum += secret;
        }
        running_sum.to_string()
    } else {
        let input = input.get();
        let mut sellers_prices = Vec::new();
        let mut sellers_diffs = Vec::new();
        for line in input.lines() {
            let init = line.parse::<u64>().unwrap();
            let mut prices = vec![extract_price(init)];
            let mut diffs = vec![0];

            let mut secret = init;
            let mut last_price = extract_price(init);
            for _ in 0..NUM_NEW_RANDS {
                secret = next_secret(secret);
                let price = extract_price(secret);
                let diff = (price as i64) - (last_price as i64);
                prices.push(price);
                diffs.push(diff);
                last_price = price;
            }
            sellers_prices.push(prices);
            sellers_diffs.push(diffs);
        }

        let mut scores: HashMap<(i64, i64, i64, i64), u64> = HashMap::new();
        sellers_prices
            .into_iter()
            .zip(sellers_diffs)
            .for_each(|(prices, diffs)| {
                diffs
                    .iter()
                    .copied()
                    .skip(1)
                    .tuple_windows::<(i64, i64, i64, i64)>()
                    .map(|seq| {
                        let idx = find_seq(&diffs, seq).unwrap();
                        (idx, seq, prices[idx])
                    })
                    .unique_by(|(idx, _, _)| *idx)
                    .for_each(|(_, seq, price)| {
                        scores.entry(seq).or_default().add_assign(price);
                    });
            });
        scores.values().max().unwrap().to_string()
    }
}

#[inline]
fn mix(num: u64, secret: u64) -> u64 {
    num ^ secret
}

#[inline]
fn prune(num: u64) -> u64 {
    // num % 16777216
    const SHIFT: usize = u64::BITS as usize - 24;
    num << SHIFT >> SHIFT
}

fn next_secret(secret: u64) -> u64 {
    let mul = secret * 64;
    let secret = prune(mix(mul, secret));
    let div = secret / 32;
    let secret = prune(mix(div, secret));
    let mul = secret * 2048;
    prune(mix(mul, secret))
}

fn extract_price(secret: u64) -> u64 {
    secret % 10
}

fn find_seq(diffs: &[i64], seq: (i64, i64, i64, i64)) -> Option<usize> {
    for i in 1..diffs.len() - 3 {
        if diffs[i] == seq.0
            && diffs[i + 1] == seq.1
            && diffs[i + 2] == seq.2
            && diffs[i + 3] == seq.3
        {
            return Some(i + 3);
        }
    }
    None
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_mix() {
        assert_eq!(mix(15, 42), 37);
    }

    #[test]
    fn test_prune() {
        assert_eq!(prune(100000000), 16113920);
    }
}
