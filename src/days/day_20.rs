use std::collections::{HashMap, HashSet};

use crate::input::{Input, Part};

// const CHEAT_THRESHOLD: usize = 0;
const CHEAT_THRESHOLD: usize = 100;
// const CHEAT_THRESHOLD: usize = 50;

const CHEAT_LEN_P1: usize = 2;
const CHEAT_LEN_P2: usize = 20;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tile {
    Wall,
    Track,
}

pub(super) fn run(input: &Input, part: Part) -> String {
    let mut map = Vec::new();
    let mut start = (0, 0);
    let mut finish = (0, 0);
    for line in input.get().lines() {
        map.push(
            line.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    '#' => Tile::Wall,
                    '.' => Tile::Track,
                    'S' => {
                        start = (x, map.len());
                        Tile::Track
                    }
                    'E' => {
                        finish = (x, map.len());
                        Tile::Track
                    }
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>(),
        );
    }
    let mut visited = vec![vec![false; map[0].len()]; map.len()];
    visited[start.1][start.0] = true;
    let min_path = find_path(start, finish, &map, 0, &mut visited);
    assert!(min_path < usize::MAX);
    let mut cheats = HashMap::new();
    let _ = find_path_with_cheat(
        start,
        finish,
        &map,
        0,
        false,
        if part == Part::One {
            CHEAT_LEN_P1 - 1
        } else {
            CHEAT_LEN_P2 - 1
        },
        (None, None),
        &mut cheats,
        min_path,
        &mut visited,
    );
    cheats.values().map(|x| x.len()).sum::<usize>().to_string()
}

fn find_path(
    start: (usize, usize),
    finish: (usize, usize),
    map: &[Vec<Tile>],
    path_len: usize,
    visited: &mut [Vec<bool>],
) -> usize {
    if start == finish {
        // print_map(map, visited);
        println!("path_len: {}", path_len);
        return path_len;
    }
    let up = (start.0, start.1 - 1);
    let down = (start.0, start.1 + 1);
    let left = (start.0 - 1, start.1);
    let right = (start.0 + 1, start.1);

    let mut min_path = usize::MAX;
    for tile in [up, down, left, right] {
        if tile.0 == usize::MAX
            || tile.0 >= map.len()
            || tile.1 == usize::MAX
            || tile.1 >= map[0].len()
        {
            continue;
        }
        if map[tile.1][tile.0] == Tile::Track && !visited[tile.1][tile.0] {
            visited[tile.1][tile.0] = true;
            min_path = min_path.min(find_path(tile, finish, map, path_len + 1, visited));
            visited[tile.1][tile.0] = false;
        }
    }
    min_path
}

#[allow(clippy::too_many_arguments)]
fn find_path_with_cheat(
    start: (usize, usize),
    finish: (usize, usize),
    map: &[Vec<Tile>],
    path_len: usize,
    cheat_mode: bool,
    cheat_left: usize,
    cheat: (Option<(usize, usize)>, Option<(usize, usize)>),
    cheats: &mut HashMap<usize, HashSet<((usize, usize), (usize, usize))>>,
    best_path_len: usize,
    visited: &mut [Vec<bool>],
) -> usize {
    if start == finish {
        let cheat_save = 84 - path_len;
        if path_len == 84 {
            return 0;
        }
        assert!(cheat.0.is_some());
        assert!(cheat.1.is_some());
        let cheat = (cheat.0.unwrap(), cheat.1.unwrap());

        let cheat_set = cheats.entry(cheat_save).or_default();
        let was_new = cheat_set.insert(cheat);

        if was_new && cheat_save == 76 {
            print_map(map, visited, cheat);
            println!("cheat_save: {cheat_save}");
        }
        return 1;
    }
    if path_len > best_path_len - CHEAT_THRESHOLD {
        return 0;
    }
    let up = (start.0, start.1.wrapping_sub(1));
    let down = (start.0, start.1 + 1);
    let left = (start.0.wrapping_sub(1), start.1);
    let right = (start.0 + 1, start.1);

    let mut possible_paths_sum = 0;
    for tile in [up, down, left, right] {
        if tile.0 == usize::MAX
            || tile.0 >= map.len()
            || tile.1 == usize::MAX
            || tile.1 >= map[0].len()
        {
            continue;
        }
        if map[tile.1][tile.0] == Tile::Track && !visited[tile.1][tile.0] {
            visited[tile.1][tile.0] = true;
            let cheat = if cheat_mode && cheat_left > 0 || cheat_mode && cheat.1.is_none() {
                (cheat.0, Some(tile))
            } else {
                cheat
            };
            let cheat_left = if cheat_mode {
                cheat_left.saturating_sub(1)
            } else {
                cheat_left
            };
            assert!(
                (cheat.0.is_none() && cheat.1.is_none())
                    || (cheat.0.is_some() && cheat.1.is_some())
            );
            possible_paths_sum += find_path_with_cheat(
                tile,
                finish,
                map,
                path_len + 1,
                cheat_mode,
                cheat_left,
                cheat,
                cheats,
                best_path_len,
                visited,
            );
            visited[tile.1][tile.0] = false;
        }
        if cheat_left > 0 && map[tile.1][tile.0] == Tile::Wall && !visited[tile.1][tile.0] {
            visited[tile.1][tile.0] = true;
            let cheat = if !cheat_mode {
                assert!(cheat.0.is_none());
                assert!(cheat.1.is_none());
                (Some(start), None)
            } else {
                cheat
            };
            possible_paths_sum += find_path_with_cheat(
                tile,
                finish,
                map,
                path_len + 1,
                true,
                cheat_left - 1,
                cheat,
                cheats,
                best_path_len,
                visited,
            );
            visited[tile.1][tile.0] = false;
        }
    }
    possible_paths_sum
}

fn print_map(
    map: &[Vec<Tile>],
    visited: &mut [Vec<bool>],
    cheat: ((usize, usize), (usize, usize)),
) {
    for (i, row) in map.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if (j, i) == cheat.0 {
                print!("?");
            } else if (j, i) == cheat.1 {
                print!("!");
            } else if visited[i][j] {
                print!("*");
            } else if *cell == Tile::Wall {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!()
    }
}
