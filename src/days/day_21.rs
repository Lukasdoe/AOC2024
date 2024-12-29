use crate::input::{Input, Part};
use itertools::Itertools;

pub(super) fn run(input: &Input, part: Part) -> String {
    let mut complexity_sum = 0;
    for code in input.get().lines() {
        let numeric_value = code
            .chars()
            .take_while(|c| c.is_numeric())
            .map(|c| c.to_digit(10).unwrap())
            .fold(0, |acc, x| acc * 10 + x);
        let code = code.chars().collect::<Vec<char>>();
        println!("Code: {:?}", code);
        let mut current_paths = traveling_robot_salesman(
            numbers_to_coords(vec!['A'])[0],
            &numbers_to_coords(code),
            true,
        );
        let required_iters = if part == Part::One { 2 } else { 25 };
        for _ in 0..required_iters {
            let shortest_path_length = current_paths.iter().map(|path| path.len()).min().unwrap();
            let len_prev = current_paths.len();
            current_paths = current_paths
                .into_iter()
                .filter(|path| path.len() == shortest_path_length)
                .take(1)
                .collect::<Vec<_>>();
            let len_post = current_paths.len();
            println!(
                "Shortest path length: {}, Paths before: {}, Paths after: {}",
                shortest_path_length, len_prev, len_post
            );
            current_paths = current_paths
                .into_iter()
                .flat_map(|path| {
                    traveling_robot_salesman(
                        directions_to_coords(vec!['A'])[0],
                        &directions_to_coords(path),
                        false,
                    )
                })
                .collect::<Vec<_>>();
        }
        let (shortest_seq_len, shortest_seq) = current_paths
            .into_iter()
            .map(|path| (path.len(), path))
            .min_by_key(|(len, _)| *len)
            .unwrap();
        let complexity = shortest_seq_len * (numeric_value as usize);
        complexity_sum += complexity;

        println!(
            "seq_len: {shortest_seq_len}, Code: {}, Complexity: {}, Path: {}",
            numeric_value,
            complexity,
            shortest_seq.iter().join("")
        );
    }
    complexity_sum.to_string()
}

fn numbers_to_coords(nums: Vec<char>) -> Vec<(u8, u8)> {
    let mut coords = Vec::new();
    for num in nums {
        let c = match num {
            '7' => (0, 0),
            '8' => (1, 0),
            '9' => (2, 0),
            '4' => (0, 1),
            '5' => (1, 1),
            '6' => (2, 1),
            '1' => (0, 2),
            '2' => (1, 2),
            '3' => (2, 2),
            '0' => (1, 3),
            'A' => (2, 3),
            _ => panic!("Invalid number"),
        };
        coords.push(c);
    }
    coords
}

fn directions_to_coords(dirs: Vec<char>) -> Vec<(u8, u8)> {
    let mut coords = Vec::new();
    for dir in dirs {
        let c = match dir {
            '^' => (1, 0),
            'A' => (2, 0),
            '<' => (0, 1),
            'v' => (1, 1),
            '>' => (2, 1),
            _ => panic!("Invalid direction"),
        };
        coords.push(c);
    }
    coords
}

fn traveling_robot_salesman(start: (u8, u8), coords: &[(u8, u8)], numeric: bool) -> Vec<Vec<char>> {
    if coords.is_empty() {
        return vec![Vec::new()];
    }
    let next = coords[0];
    let mut paths = find_path_rec(start, next, numeric).1;
    paths.iter_mut().for_each(|p| p.push('A'));
    paths
        .into_iter()
        .flat_map(|p| {
            traveling_robot_salesman(next, &coords[1..], numeric)
                .into_iter()
                .map(|mut sub_path| {
                    let mut p = p.clone();
                    p.append(&mut sub_path);
                    p
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

fn find_path_rec(start: (u8, u8), dest: (u8, u8), numeric: bool) -> (usize, Vec<Vec<char>>) {
    if start == dest {
        return (0, vec![Vec::new()]);
    }
    if (numeric && start == (0, 3)) || (!numeric && start == (0, 0)) {
        return (usize::MAX, Vec::new());
    }
    let mut min_paths = Vec::new();
    let mut min_len = usize::MAX;
    if start.0 < dest.0 {
        let (len, mut paths) = find_path_rec((start.0 + 1, start.1), dest, numeric);
        paths.iter_mut().for_each(|p| p.insert(0, '>'));
        let len = len.saturating_add(1);
        if len < min_len {
            min_len = len;
            min_paths = paths;
        } else if len == min_len {
            min_paths.append(&mut paths);
        }
    }
    if start.1 < dest.1 {
        let (len, mut paths) = find_path_rec((start.0, start.1 + 1), dest, numeric);
        paths.iter_mut().for_each(|p| p.insert(0, 'v'));
        let len = len.saturating_add(1);
        if len < min_len {
            min_len = len;
            min_paths = paths;
        } else if len == min_len {
            min_paths.append(&mut paths);
        }
    }
    if start.0 > dest.0 {
        let (len, mut paths) = find_path_rec((start.0 - 1, start.1), dest, numeric);
        paths.iter_mut().for_each(|p| p.insert(0, '<'));
        let len = len.saturating_add(1);
        if len < min_len {
            min_len = len;
            min_paths = paths;
        } else if len == min_len {
            min_paths.append(&mut paths);
        }
    }
    if start.1 > dest.1 {
        let (len, mut paths) = find_path_rec((start.0, start.1 - 1), dest, numeric);
        paths.iter_mut().for_each(|p| p.insert(0, '^'));
        let len = len.saturating_add(1);
        if len < min_len {
            min_len = len;
            min_paths = paths;
        } else if len == min_len {
            min_paths.append(&mut paths);
        }
    }
    (min_len, min_paths)
}
