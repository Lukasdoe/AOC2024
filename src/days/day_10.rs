use crate::input::{Input, Part};
use std::collections::HashSet;

pub(super) fn run(input: &Input, part: Part) -> String {
    let mut map = Vec::new();
    for line in input.get().lines() {
        let mut row = Vec::new();
        for c in line.chars() {
            if c == '.' {
                row.push(100000);
                continue;
            }
            row.push(c.to_digit(10).unwrap() as u64);
        }
        map.push(row);
    }

    let mut trailheadsum = 0;
    for row in 0..map.len() {
        for col in 0..map[0].len() {
            if map[row][col] == 0 {
                if part == Part::One {
                    let th_score = rec_trail(row, col, &map);
                    trailheadsum += th_score.len() as u64;
                } else {
                    trailheadsum += rec_trail2(row, col, &map);
                }
            }
        }
    }

    trailheadsum.to_string()
}

fn rec_trail(row: usize, col: usize, map: &[Vec<u64>]) -> HashSet<(usize, usize)> {
    let cur = map[row][col];
    if map[row][col] == 9 {
        return HashSet::from([(row, col)]);
    }

    let mut trails = HashSet::new();
    if row + 1 < map.len() && map[row + 1][col] == cur + 1 {
        trails.extend(rec_trail(row + 1, col, map));
    }
    if col + 1 < map[0].len() && map[row][col + 1] == cur + 1 {
        trails.extend(rec_trail(row, col + 1, map));
    }
    if row > 0 && map[row - 1][col] == cur + 1 {
        trails.extend(rec_trail(row - 1, col, map));
    }
    if col > 0 && map[row][col - 1] == cur + 1 {
        trails.extend(rec_trail(row, col - 1, map));
    }
    trails
}

fn rec_trail2(row: usize, col: usize, map: &[Vec<u64>]) -> u64 {
    let cur = map[row][col];
    if map[row][col] == 9 {
        return 1;
    }

    let mut trails = 0;
    if row + 1 < map.len() && map[row + 1][col] == cur + 1 {
        trails += rec_trail2(row + 1, col, map);
    }
    if col + 1 < map[0].len() && map[row][col + 1] == cur + 1 {
        trails += rec_trail2(row, col + 1, map);
    }
    if row > 0 && map[row - 1][col] == cur + 1 {
        trails += rec_trail2(row - 1, col, map);
    }
    if col > 0 && map[row][col - 1] == cur + 1 {
        trails += rec_trail2(row, col - 1, map);
    }
    trails
}
