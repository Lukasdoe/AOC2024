use crate::input::{Input, Part};

pub(super) fn run(input: &Input, part: Part) -> u64 {
    let lines = input
        .get()
        .lines()
        .map(|l| {
            l.split(" ")
                .map(|n| n.parse::<u64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    if part == Part::One {
        let mut count_unsafe = 0;
        for line in lines.iter() {
            let increasing = line[0] < line[1];
            for pairs in line.windows(2) {
                if (increasing && pairs[0] > pairs[1]) || (!increasing && pairs[0] < pairs[1]) {
                    count_unsafe += 1;
                    break;
                }
                let diff = pairs[0].abs_diff(pairs[1]);
                if !(1..=3).contains(&diff) {
                    count_unsafe += 1;
                    break;
                }
            }
        }
        (lines.len() - count_unsafe) as u64
    } else {
        let mut count_safe = 0;
        for line in lines.iter() {
            let mut is_safe = true;
            let increasing = line[0] < line[1];
            for pairs in line.windows(2) {
                if (increasing && pairs[0] > pairs[1]) || (!increasing && pairs[0] < pairs[1]) {
                    is_safe = false;
                    break;
                }
                let diff = pairs[0].abs_diff(pairs[1]);
                if !(1..=3).contains(&diff) {
                    is_safe = false;
                    break;
                }
            }
            if is_safe {
                count_safe += 1;
            } else {
                for i in 0..line.len() {
                    is_safe = true;
                    let mut modified_line = line.clone();
                    modified_line.remove(i);

                    let increasing = modified_line[0] < modified_line[1];
                    for pairs in modified_line.windows(2) {
                        if (increasing && pairs[0] > pairs[1])
                            || (!increasing && pairs[0] < pairs[1])
                        {
                            is_safe = false;
                            break;
                        }
                        let diff = pairs[0].abs_diff(pairs[1]);
                        if !(1..=3).contains(&diff) {
                            is_safe = false;
                            break;
                        }
                    }
                    if is_safe {
                        count_safe += 1;
                        break;
                    }
                }
            }
        }
        count_safe as u64
    }
}
