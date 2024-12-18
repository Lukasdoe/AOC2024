use std::collections::BinaryHeap;

use crate::input::{Input, Part};

const START_POS: (usize, usize) = (0, 0);
const DEST_POS: (usize, usize) = (70, 70);

pub(super) fn run(input: &Input, part: Part) -> String {
    let start = START_POS;
    let dest = DEST_POS;

    let mut bytes = Vec::new();
    for line in input.get().lines() {
        let (x, y) = line.split_once(",").unwrap();
        bytes.push((x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap()));
    }

    let mut map = vec![vec![false; 71]; 71];
    for (x, y) in bytes.iter().take(1024) {
        map[*y][*x] = true;
    }
    if part == Part::One {
        let path = find_path(start, dest, &map);
        print_map(&map, &path);
        (path.len()).to_string()
    } else {
        let mut last_byte = (0, 0);
        let mut bytes_iter = bytes.iter().skip(1024);
        while !find_path(start, dest, &map).is_empty() {
            last_byte = *bytes_iter.next().unwrap();
            map[last_byte.1][last_byte.0] = true;
        }
        format!("{},{}", last_byte.0, last_byte.1)
    }
}

fn find_path(
    start: (usize, usize),
    dest: (usize, usize),
    map: &[Vec<bool>],
) -> Vec<(usize, usize)> {
    let mut visited = vec![vec![false; map.len()]; map[0].len()];
    let mut queue = BinaryHeap::new();
    queue.push((std::cmp::Reverse(0), start.0, start.1, vec![]));
    visited[start.1][start.0] = true;

    while !queue.is_empty() {
        let (_, x, y, path) = queue.pop().unwrap();
        if (x, y) == dest {
            return path;
        }

        for (dx, dy) in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            if nx < 0 || ny < 0 || nx >= map[0].len() as i32 || ny >= map.len() as i32 {
                continue;
            }

            let nx = nx as usize;
            let ny = ny as usize;
            if visited[ny][nx] || map[ny][nx] {
                continue;
            }

            visited[ny][nx] = true;
            let mut path = path.clone();
            path.push((nx, ny));
            queue.push((std::cmp::Reverse(path.len()), nx, ny, path));
        }
    }
    Vec::new()
}

fn print_map(map: &[Vec<bool>], path: &[(usize, usize)]) {
    for (y, row) in map.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if path.contains(&(x, y)) {
                print!("O");
            } else if *cell {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}
