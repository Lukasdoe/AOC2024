use std::{path::PathBuf, sync::LazyLock};

use crate::{aoc_api::AoCAPI, days::Day};
use soup::{NodeExt, QueryBuilderExt};

static RESOURCES: LazyLock<PathBuf> = LazyLock::new(|| {
    std::env!("CARGO_MANIFEST_DIR")
        .parse::<PathBuf>()
        .unwrap()
        .join("resources")
});

struct AoCSource {
    source: String,
    solution: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Part {
    One,
    Two,
}

impl From<Part> for u8 {
    fn from(part: Part) -> u8 {
        match part {
            Part::One => 1,
            Part::Two => 2,
        }
    }
}

pub struct Input {
    source: AoCSource,
}

impl Input {
    pub fn new(day: Day, debug: bool, part: Part) -> Self {
        let source = if debug {
            Self::load_example(day, part)
        } else {
            Self::load_source(day, part)
        };
        Self { source }
    }

    #[allow(dead_code)]
    pub fn custom(source: String, solution: String) -> Self {
        Self {
            source: AoCSource {
                source,
                solution: Some(solution),
            },
        }
    }

    pub fn get(&self) -> &str {
        self.source.source.as_str()
    }

    pub fn get_solution(&self) -> Option<String> {
        self.source.solution.clone()
    }

    fn load_source(day: Day, part: Part) -> AoCSource {
        let day_num: u8 = day.into();
        let part_num: u8 = part.into();
        let source_file_path = RESOURCES.join(format!("day_{}_{}_src.txt", day_num, part_num));
        let sol_file_path = RESOURCES.join(format!("day_{}_{}_sol.txt", day_num, part_num));
        if source_file_path.exists() {
            let source = std::fs::read_to_string(&source_file_path).unwrap();
            let solution = Some(
                std::fs::read_to_string(&sol_file_path)
                    .unwrap()
                    .trim()
                    .to_owned(),
            );
            return AoCSource { source, solution };
        }
        let source = AoCAPI::get_input(day);
        std::fs::write(&source_file_path, source.as_bytes()).unwrap();
        std::fs::write(&sol_file_path, []).unwrap();
        AoCSource {
            source,
            solution: None,
        }
    }

    fn load_example(day: Day, part: Part) -> AoCSource {
        let day_num: u8 = day.into();
        let part_num: u8 = part.into();
        let dbg_source_file_path = RESOURCES.join(format!("day_{}_{}_dbg.txt", day_num, part_num));
        let dbg_sol_file_path = RESOURCES.join(format!("day_{}_{}_dbg_sol.txt", day_num, part_num));
        if dbg_source_file_path.exists() {
            let source = std::fs::read_to_string(&dbg_source_file_path).unwrap();
            let solution = std::fs::read_to_string(&dbg_sol_file_path).unwrap();
            return AoCSource {
                source,
                solution: Some(solution.trim().parse().unwrap()),
            };
        }

        let html = AoCAPI::get_page(day);
        let soup = soup::Soup::new(&html);
        let articles = soup.tag("article").find_all().collect::<Vec<_>>();
        let article_children = articles[0].children.borrow();
        let mut example_source = None;
        for i in 1..article_children.len() {
            if article_children[i].tag("code").find().is_some() {
                let pred_text = article_children[i - 2].text();
                if pred_text.contains("example") && pred_text.contains(":") {
                    example_source = Some(article_children[i].text());
                    break;
                }
            }
        }
        if example_source.is_none() {
            panic!("Couldn't find example source");
        }
        println!("Found example:\n{}", example_source.as_ref().unwrap());

        let article = if part == Part::One || articles.len() == 1 {
            articles[0].clone()
        } else {
            articles[1].clone()
        };

        let code_tags = article.tag("code").find_all().collect::<Vec<_>>();
        for tag in code_tags.iter().rev() {
            if let Some(em) = tag.tag("em").find() {
                let solution = em.text().trim().to_owned();
                println!("Found example solution: {}", solution);
                let res = AoCSource {
                    source: example_source.unwrap(),
                    solution: Some(solution.clone()),
                };
                std::fs::write(dbg_source_file_path, res.source.as_bytes()).unwrap();
                std::fs::write(dbg_sol_file_path, solution.as_bytes()).unwrap();
                return res;
            }
        }
        unreachable!()
    }
}
