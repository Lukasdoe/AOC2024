use std::process::exit;

use clap::Parser;
use days::Day;
use input::{Input, Part};

mod aoc_api;
mod days;
mod input;
mod tests;

#[derive(Parser)]
#[command(name = "Advent of Code")]
#[command(about = "Run a specific day's solution", long_about = None)]
struct Args {
    /// Day to run
    day: u8,

    /// Only run part 2
    #[clap(short = '2', default_value = "false")]
    only_part_2: bool,

    /// Submit the solution
    #[clap(short = 's', default_value = "false")]
    submit: bool,
}

fn main() {
    let args = Args::parse();
    let day = Day::try_from(args.day).expect("Invalid day");

    for part in [Part::One, Part::Two] {
        println!("----------------------------------------");
        if args.only_part_2 && part == Part::One {
            continue;
        }
        println!("Retrieving debug input.");
        let input = Input::new(day, true, part);
        // println!("Running day {}.", args.day);
        let result = days::run_day(day, &input, part);
        let expected = input.get_solution().unwrap();
        if result == expected {
            println!("Result (example): {} vs. {} => PASS", result, expected,);
        } else {
            println!("Result (example): {} vs. {} => FAIL", result, expected,);
            exit(1);
        }

        // println!("Retrieving real input.");
        let input = Input::new(day, false, part);
        // println!("Running day {}.", args.day);
        let (result, time) = timeit(|| days::run_day(day, &input, part));
        println!(
            "Result (real): {} in {}ms / {}us",
            result,
            time.as_millis(),
            time.as_micros()
        );
        if !args.submit {
            continue;
        }
        print!("Submitting solution... ");
        let submitted = aoc_api::AoCAPI::submit_solution(day, part, result);
        match submitted {
            aoc_api::SolutionResult::Correct => println!("Success!"),
            aoc_api::SolutionResult::Incorrect => {
                println!("Failed!");
                exit(1)
            }
            aoc_api::SolutionResult::RateLimited => {
                println!("Rate limited!");
                exit(1)
            }
            aoc_api::SolutionResult::AlreadyCompleted => {
                println!("Already completed!");
                exit(1)
            }
        }
    }
    println!("----------------------------------------");
}

fn timeit<R>(f: impl FnOnce() -> R) -> (R, std::time::Duration) {
    let start = std::time::Instant::now();
    let r = f();
    let end = std::time::Instant::now();
    (r, end - start)
}
