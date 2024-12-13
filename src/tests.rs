#[cfg(test)]
use crate::{
    days::{self, Day},
    input::{Input, Part},
};

#[cfg(test)]
fn test(day: Day, part: Part) {
    let now = chrono::Local::now().date_naive();
    let first_of_december = chrono::NaiveDate::from_ymd_opt(2024, 12, 1).unwrap();
    if (day as u8) > ((now - first_of_december).num_days() as u8) {
        return;
    }

    let input = Input::new(day, true, part);
    let result = days::run_day(day, &input, part);
    assert_eq!(result, input.get_solution().unwrap());

    let input = Input::new(day, false, part);
    let result = days::run_day(day, &input, part);
    assert_eq!(result, input.get_solution().unwrap());
}

#[cfg(test)]
fn test_part_one(day: Day) {
    test(day, Part::One);
}

#[cfg(test)]
fn test_part_two(day: Day) {
    test(day, Part::Two);
}

#[cfg(test)]
mod day_01 {
    use super::*;

    #[test]
    fn part_one() {
        test_part_one(Day::Day1);
    }

    #[test]
    fn part_two() {
        test_part_two(Day::Day1);
    }
}
#[cfg(test)]
mod day_02 {
    use super::*;

    #[test]
    fn part_one() {
        test_part_one(Day::Day2);
    }

    #[test]
    fn part_two() {
        test_part_two(Day::Day2);
    }
}

#[cfg(test)]
mod day_03 {
    use super::*;

    #[test]
    fn part_one() {
        test_part_one(Day::Day3);
    }

    #[test]
    fn part_two() {
        test_part_two(Day::Day3);
    }
}

#[cfg(test)]
mod day_04 {
    use super::*;

    #[test]
    fn part_one() {
        test_part_one(Day::Day4);
    }

    #[test]
    fn part_two() {
        test_part_two(Day::Day4);
    }
}

#[cfg(test)]
mod day_05 {
    use super::*;

    #[test]
    fn part_one() {
        test_part_one(Day::Day5);
    }

    #[test]
    fn part_two() {
        test_part_two(Day::Day5);
    }
}
#[cfg(test)]
mod day_06 {
    use super::*;

    #[test]
    fn part_one() {
        test_part_one(Day::Day6);
    }

    #[test]
    fn part_two() {
        test_part_two(Day::Day6);
    }
}

#[cfg(test)]
mod day_07 {
    use super::*;

    #[test]
    fn part_one() {
        test_part_one(Day::Day7);
    }

    #[test]
    fn part_two() {
        test_part_two(Day::Day7);
    }
}

#[cfg(test)]
mod day_08 {
    use super::*;

    #[test]
    fn part_one() {
        test_part_one(Day::Day8);
    }

    #[test]
    fn part_two() {
        test_part_two(Day::Day8);
    }
}

#[cfg(test)]
mod day_09 {
    use super::*;

    #[test]
    fn part_one() {
        test_part_one(Day::Day9);
    }

    #[test]
    fn part_two() {
        test_part_two(Day::Day9);
    }
}

#[cfg(test)]
mod day_10 {
    use super::*;

    #[test]
    fn part_one() {
        test_part_one(Day::Day10);
    }

    #[test]
    fn part_two() {
        test_part_two(Day::Day10);
    }
}
#[cfg(test)]
mod day_11 {
    use super::*;

    #[test]
    fn part_one() {
        test_part_one(Day::Day11);
    }

    #[test]
    fn part_two() {
        test_part_two(Day::Day11);
    }
}

#[cfg(test)]
mod day_12 {
    use super::*;

    #[test]
    fn part_one() {
        test_part_one(Day::Day12);
    }

    #[test]
    fn part_two() {
        test_part_two(Day::Day12);
    }
}

#[cfg(test)]
mod day_13 {
    use super::*;

    #[test]
    fn part_one() {
        test_part_one(Day::Day13);
    }

    #[test]
    fn part_two() {
        test_part_two(Day::Day13);
    }
}

#[cfg(test)]
mod day_14 {
    use super::*;

    #[test]
    fn part_one() {
        test_part_one(Day::Day14);
    }

    #[test]
    fn part_two() {
        test_part_two(Day::Day14);
    }
}

#[cfg(test)]
mod day_15 {
    use super::*;

    #[test]
    fn part_one() {
        test_part_one(Day::Day15);
    }

    #[test]
    fn part_two() {
        test_part_two(Day::Day15);
    }
}

#[cfg(test)]
mod day_16 {
    use super::*;

    #[test]
    fn part_one() {
        test_part_one(Day::Day16);
    }

    #[test]
    fn part_two() {
        test_part_two(Day::Day16);
    }
}

#[cfg(test)]
mod day_17 {
    use super::*;

    #[test]
    fn part_one() {
        test_part_one(Day::Day17);
    }

    #[test]
    fn part_two() {
        test_part_two(Day::Day17);
    }
}

#[cfg(test)]
mod day_18 {
    use super::*;

    #[test]
    fn part_one() {
        test_part_one(Day::Day18);
    }

    #[test]
    fn part_two() {
        test_part_two(Day::Day18);
    }
}

#[cfg(test)]
mod day_19 {
    use super::*;

    #[test]
    fn part_one() {
        test_part_one(Day::Day19);
    }

    #[test]
    fn part_two() {
        test_part_two(Day::Day19);
    }
}

#[cfg(test)]
mod day_20 {
    use super::*;

    #[test]
    fn part_one() {
        test_part_one(Day::Day20);
    }

    #[test]
    fn part_two() {
        test_part_two(Day::Day20);
    }
}

#[cfg(test)]
mod day_21 {
    use super::*;

    #[test]
    fn part_one() {
        test_part_one(Day::Day21);
    }

    #[test]
    fn part_two() {
        test_part_two(Day::Day21);
    }
}

#[cfg(test)]
mod day_22 {
    use super::*;

    #[test]
    fn part_one() {
        test_part_one(Day::Day22);
    }

    #[test]
    fn part_two() {
        test_part_two(Day::Day22);
    }
}

#[cfg(test)]
mod day_23 {
    use super::*;

    #[test]
    fn part_one() {
        test_part_one(Day::Day23);
    }

    #[test]
    fn part_two() {
        test_part_two(Day::Day23);
    }
}

#[cfg(test)]
mod day_24 {
    use super::*;

    #[test]
    fn part_one() {
        test_part_one(Day::Day24);
    }

    #[test]
    fn part_two() {
        test_part_two(Day::Day24);
    }
}

#[cfg(test)]
mod day_25 {
    use super::*;

    #[test]
    fn part_one() {
        test_part_one(Day::Day25);
    }

    #[test]
    fn part_two() {
        test_part_two(Day::Day25);
    }
}
