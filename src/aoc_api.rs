use std::sync::LazyLock;

use soup::{NodeExt, QueryBuilderExt};
use ureq::Response;

use crate::{days::Day, input::Part};

static AOC_SESSION_COOKIE: LazyLock<String> = LazyLock::new(|| {
    std::env::var("AOC_SESSION_COOKIE").expect("AOC_SESSION_COOKIE environment variable not set")
});

pub struct AoCAPI;

#[derive(Debug, PartialEq, Eq)]
pub enum SolutionResult {
    Correct,
    Incorrect,
    RateLimited,
    AlreadyCompleted,
}

impl AoCAPI {
    pub fn get_input(day: Day) -> String {
        let day_num: u8 = day.into();
        let url = format!("https://adventofcode.com/2024/day/{day_num}/input",);
        AoCAPI::get(&url)
            .into_string()
            .expect("Failed to read input")
    }

    pub fn get_page(day: Day) -> String {
        let day_num: u8 = day.into();
        let url = format!("https://adventofcode.com/2024/day/{day_num}",);
        AoCAPI::get(&url)
            .into_string()
            .expect("Failed to read page")
    }

    fn get(url: &str) -> Response {
        ureq::get(url)
            .set(
                "Cookie",
                &format!("session={}", LazyLock::force(&AOC_SESSION_COOKIE)),
            )
            .call()
            .expect("Failed to fetch page")
    }

    fn post(url: &str, form: &[(&str, &str)]) -> Response {
        ureq::post(url)
            .set(
                "Cookie",
                &format!("session={}", LazyLock::force(&AOC_SESSION_COOKIE)),
            )
            .send_form(form)
            .expect("Failed to submit form")
    }

    pub fn submit_solution(day: Day, part: Part, solution: u64) -> SolutionResult {
        let day_num: u8 = day.into();
        let url = format!("https://adventofcode.com/2024/day/{day_num}/answer",);
        let level: u8 = part.into();
        let level_str = level.to_string();
        let solution_str = solution.to_string();
        let form = [
            ("level", level_str.as_str()),
            ("answer", solution_str.as_str()),
        ];
        let response = AoCAPI::post(&url, &form);
        let response_text =
            soup::Soup::new(&response.into_string().expect("Failed to read response"))
                .tag("article")
                .find()
                .unwrap()
                .text();
        if response_text.contains("That's the right answer") {
            SolutionResult::Correct
        } else if response_text.contains("That's not the right answer") {
            SolutionResult::Incorrect
        } else if response_text.contains("You gave an answer too recently") {
            SolutionResult::RateLimited
        } else if response_text.contains("You don't seem to be solving the right level.") {
            SolutionResult::AlreadyCompleted
        } else {
            panic!("Unexpected response: {}", response_text)
        }
    }
}
