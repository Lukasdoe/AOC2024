use crate::input::{Input, Part};

pub(super) fn run(input: &Input, part: Part) -> String {
    let mut sum = 0;
    let s = input.get();
    if part == Part::One {
        regex::Regex::new(r#"mul\((\d+),(\d+)\)"#)
            .unwrap()
            .captures_iter(s)
            .for_each(|cap| {
                let n1 = cap[1].parse::<u64>().unwrap();
                let n2 = cap[2].parse::<u64>().unwrap();
                sum += n1 * n2;
            });
        sum.to_string()
    } else {
        let mut enabled = true;
        regex::Regex::new(r#"(?:mul\((\d+),(\d+)\))|(?:do\(\))|(?:don't\(\))"#)
            .unwrap()
            .captures_iter(s)
            .for_each(|cap| {
                if &cap[0] == "do()" {
                    enabled = true;
                } else if &cap[0] == "don't()" {
                    enabled = false;
                } else if enabled {
                    let n1 = cap[1].parse::<u64>().unwrap();
                    let n2 = cap[2].parse::<u64>().unwrap();
                    sum += n1 * n2;
                }
            });
        sum.to_string()
    }
}
