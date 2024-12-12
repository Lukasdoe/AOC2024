use crate::input::{Input, Part};

pub(super) fn run(input: &Input, part: Part) -> u64 {
    let mut inputs = vec![];
    for line in input.get().lines() {
        let (target, nums) = line.split_once(": ").unwrap();
        let target = target.parse::<u64>().unwrap();
        let nums = nums
            .split(" ")
            .map(|num| num.parse::<u64>().unwrap())
            .collect::<Vec<_>>();
        inputs.push((target, nums));
    }

    let mut count = 0;
    for input in inputs {
        let (target, nums) = input;
        if part == Part::One {
            if find_rec(target, nums[0], &nums, 1) {
                count += target;
            }
        } else if find_rec2(target, nums[0], &nums, 1) {
            count += target;
        }
    }
    count
}

fn find_rec(target: u64, n: u64, nums: &[u64], i: usize) -> bool {
    if target == n {
        return true;
    }
    if n > target {
        return false;
    }
    if i == nums.len() {
        return false;
    }
    find_rec(target, n * nums[i], nums, i + 1) || find_rec(target, n + nums[i], nums, i + 1)
}

fn find_rec2(target: u64, n: u64, nums: &[u64], i: usize) -> bool {
    if target == n && i == nums.len() {
        return true;
    }
    if n > target || i >= nums.len() {
        return false;
    }
    find_rec2(target, n * nums[i], nums, i + 1)
        || find_rec2(target, n + nums[i], nums, i + 1)
        || find_rec2(
            target,
            n * 10_u64.pow((nums[i] as f64).log10().floor() as u32 + 1) + nums[i],
            nums,
            i + 1,
        )
}
