use crate::input::{Input, Part};

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;
mod day_10;
mod day_11;
mod day_12;
mod day_13;
mod day_14;
mod day_15;
mod day_16;
mod day_17;
mod day_18;
mod day_19;
mod day_20;
mod day_21;
mod day_22;
mod day_23;
mod day_24;
mod day_25;

#[derive(Clone, Copy)]
pub enum Day {
    Day1,
    Day2,
    Day3,
    Day4,
    Day5,
    Day6,
    Day7,
    Day8,
    Day9,
    Day10,
    Day11,
    Day12,
    Day13,
    Day14,
    Day15,
    Day16,
    Day17,
    Day18,
    Day19,
    Day20,
    Day21,
    Day22,
    Day23,
    Day24,
    Day25,
}

impl TryFrom<u8> for Day {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Day::Day1),
            2 => Ok(Day::Day2),
            3 => Ok(Day::Day3),
            4 => Ok(Day::Day4),
            5 => Ok(Day::Day5),
            6 => Ok(Day::Day6),
            7 => Ok(Day::Day7),
            8 => Ok(Day::Day8),
            9 => Ok(Day::Day9),
            10 => Ok(Day::Day10),
            11 => Ok(Day::Day11),
            12 => Ok(Day::Day12),
            13 => Ok(Day::Day13),
            14 => Ok(Day::Day14),
            15 => Ok(Day::Day15),
            16 => Ok(Day::Day16),
            17 => Ok(Day::Day17),
            18 => Ok(Day::Day18),
            19 => Ok(Day::Day19),
            20 => Ok(Day::Day20),
            21 => Ok(Day::Day21),
            22 => Ok(Day::Day22),
            23 => Ok(Day::Day23),
            24 => Ok(Day::Day24),
            25 => Ok(Day::Day25),
            _ => Err(()),
        }
    }
}

impl From<Day> for u8 {
    fn from(day: Day) -> u8 {
        match day {
            Day::Day1 => 1,
            Day::Day2 => 2,
            Day::Day3 => 3,
            Day::Day4 => 4,
            Day::Day5 => 5,
            Day::Day6 => 6,
            Day::Day7 => 7,
            Day::Day8 => 8,
            Day::Day9 => 9,
            Day::Day10 => 10,
            Day::Day11 => 11,
            Day::Day12 => 12,
            Day::Day13 => 13,
            Day::Day14 => 14,
            Day::Day15 => 15,
            Day::Day16 => 16,
            Day::Day17 => 17,
            Day::Day18 => 18,
            Day::Day19 => 19,
            Day::Day20 => 20,
            Day::Day21 => 21,
            Day::Day22 => 22,
            Day::Day23 => 23,
            Day::Day24 => 24,
            Day::Day25 => 25,
        }
    }
}

pub fn run_day(day: Day, input: &Input, part: Part) -> u64 {
    match day {
        Day::Day1 => day_01::run(input, part),
        Day::Day2 => day_02::run(input, part),
        Day::Day3 => day_03::run(input, part),
        Day::Day4 => day_04::run(input, part),
        Day::Day5 => day_05::run(input, part),
        Day::Day6 => day_06::run(input, part),
        Day::Day7 => day_07::run(input, part),
        Day::Day8 => day_08::run(input, part),
        Day::Day9 => day_09::run(input, part),
        Day::Day10 => day_10::run(input, part),
        Day::Day11 => day_11::run(input, part),
        Day::Day12 => day_12::run(input, part),
        Day::Day13 => day_13::run(input, part),
        Day::Day14 => day_14::run(input, part),
        Day::Day15 => day_15::run(input, part),
        Day::Day16 => day_16::run(input, part),
        Day::Day17 => day_17::run(input, part),
        Day::Day18 => day_18::run(input, part),
        Day::Day19 => day_19::run(input, part),
        Day::Day20 => day_20::run(input, part),
        Day::Day21 => day_21::run(input, part),
        Day::Day22 => day_22::run(input, part),
        Day::Day23 => day_23::run(input, part),
        Day::Day24 => day_24::run(input, part),
        Day::Day25 => day_25::run(input, part),
    }
}
