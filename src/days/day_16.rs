use crate::input::{Input, Part};
use itertools::Itertools;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Wall,
    Empty,
}

pub(super) fn run(input: &Input, part: Part) -> String {
    let mut map = Vec::new();
    let mut start = (0, 0);
    let mut end = (0, 0);
    for line in input.get().lines() {
        map.push(
            line.chars()
                .enumerate()
                .map(|(i, c)| match c {
                    '#' => Tile::Wall,
                    '.' => Tile::Empty,
                    'S' => {
                        start = (map.len(), i);
                        Tile::Empty
                    }
                    'E' => {
                        end = (map.len(), i);
                        Tile::Empty
                    }
                    _ => panic!("Invalid tile"),
                })
                .collect::<Vec<_>>(),
        );
    }
    let mut mem = HashMap::new();
    let (res, path) = find_path_rec(
        &map,
        start,
        end,
        (start.0, start.1 - 1),
        0,
        &mut mem,
        Vec::new(),
    );
    print_map(&map, &path);
    if part == Part::One {
        res.to_string()
    } else {
        (path.iter().unique().count() as u64 + 1).to_string()
    }
}

fn find_path_rec(
    map: &[Vec<Tile>],
    start: (usize, usize),
    end: (usize, usize),
    prev: (usize, usize),
    cost: usize,
    mem: &mut HashMap<(usize, usize), usize>,
    mut path: Vec<(usize, usize)>,
) -> (usize, Vec<(usize, usize)>) {
    path.push(start);
    if start == end {
        return (cost, path);
    }
    if mem.contains_key(&start) && mem[&start] < cost {
        return (usize::MAX, path);
    }
    mem.insert(start, cost);

    let mut res = usize::MAX;
    let mut res_path = Vec::new();

    let up = (start.0 - 1, start.1);
    let down = (start.0 + 1, start.1);
    let left = (start.0, start.1 - 1);
    let right = (start.0, start.1 + 1);

    if up != prev && map[up.0][up.1] != Tile::Wall {
        let cost = cost + if prev == down { 1 } else { 1001 };
        let (sub_res, mut sub_path) = find_path_rec(map, up, end, start, cost, mem, path.clone());
        match sub_res.cmp(&res) {
            std::cmp::Ordering::Less => {
                res = sub_res;
                res_path = sub_path;
            }
            std::cmp::Ordering::Equal => {
                res_path.append(&mut sub_path);
            }
            _ => {}
        }
    }
    if down != prev && map[down.0][down.1] != Tile::Wall {
        let cost = cost + if prev == up { 1 } else { 1001 };
        let (sub_res, mut sub_path) = find_path_rec(map, down, end, start, cost, mem, path.clone());
        match sub_res.cmp(&res) {
            std::cmp::Ordering::Less => {
                res = sub_res;
                res_path = sub_path;
            }
            std::cmp::Ordering::Equal => {
                res_path.append(&mut sub_path);
            }
            _ => {}
        }
    }
    if left != prev && map[left.0][left.1] != Tile::Wall {
        let cost = cost + if prev == right { 1 } else { 1001 };
        let (sub_res, mut sub_path) = find_path_rec(map, left, end, start, cost, mem, path.clone());
        match sub_res.cmp(&res) {
            std::cmp::Ordering::Less => {
                res = sub_res;
                res_path = sub_path;
            }
            std::cmp::Ordering::Equal => {
                res_path.append(&mut sub_path);
            }
            _ => {}
        }
    }
    if right != prev && map[right.0][right.1] != Tile::Wall {
        let cost = cost + if prev == left { 1 } else { 1001 };
        let (sub_res, mut sub_path) =
            find_path_rec(map, right, end, start, cost, mem, path.clone());
        match sub_res.cmp(&res) {
            std::cmp::Ordering::Less => {
                res = sub_res;
                res_path = sub_path;
            }
            std::cmp::Ordering::Equal => {
                res_path.append(&mut sub_path);
            }
            _ => {}
        }
    }
    (res, res_path)
}

fn print_map(map: &[Vec<Tile>], path: &[(usize, usize)]) {
    for (y, row) in map.iter().enumerate() {
        for (x, item) in row.iter().enumerate() {
            if path.contains(&(y, x)) {
                print!("O");
            } else {
                match item {
                    Tile::Wall => print!("#"),
                    Tile::Empty => print!("."),
                }
            }
        }
        println!();
    }
}
