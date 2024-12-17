use std::iter::once;

use crate::input::{Input, Part};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    BoxL,
    BoxR,
    Wall,
    Empty,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub(super) fn run(input: &Input, part: Part) -> String {
    let mut map_mode = true;
    let mut map = Vec::new();
    let mut robot = (0, 0);
    let mut sequence = Vec::new();
    for line in input.get().lines() {
        if map_mode {
            if line.is_empty() {
                map_mode = false;
                continue;
            }
            if part == Part::Two {
                map.push(
                    line.chars()
                        .enumerate()
                        .map(|(i, c)| match c {
                            '#' => Tile::Wall,
                            'O' => Tile::BoxL,
                            '.' => Tile::Empty,
                            '@' => {
                                robot = (map.len(), i * 2);
                                Tile::Empty
                            }
                            _ => panic!("Invalid tile"),
                        })
                        .flat_map(|t| match t {
                            Tile::BoxL => once(Tile::BoxL).chain(once(Tile::BoxR)),
                            _ => once(t).chain(once(t)),
                        })
                        .collect::<Vec<_>>(),
                );
            } else {
                map.push(
                    line.chars()
                        .enumerate()
                        .map(|(i, c)| match c {
                            '#' => Tile::Wall,
                            'O' => Tile::BoxL,
                            '.' => Tile::Empty,
                            '@' => {
                                robot = (map.len(), i);
                                Tile::Empty
                            }
                            _ => panic!("Invalid tile"),
                        })
                        .collect::<Vec<_>>(),
                );
            }
        } else {
            sequence.append(
                &mut line
                    .chars()
                    .map(|s| match s {
                        '^' => Direction::Up,
                        'v' => Direction::Down,
                        '<' => Direction::Left,
                        '>' => Direction::Right,
                        _ => panic!("Invalid direction"),
                    })
                    .collect::<Vec<_>>(),
            );
        }
    }
    // print_map(&map, robot);

    for instruction in sequence {
        let next_pos = match instruction {
            Direction::Up => (robot.0 - 1, robot.1),
            Direction::Down => (robot.0 + 1, robot.1),
            Direction::Left => (robot.0, robot.1 - 1),
            Direction::Right => (robot.0, robot.1 + 1),
        };
        match map[next_pos.0][next_pos.1] {
            Tile::Wall => continue,
            Tile::BoxL | Tile::BoxR => {
                if part == Part::One {
                    if !push_boxes1(next_pos, instruction, &mut map) {
                        continue;
                    }
                } else if !push_boxes2(next_pos, instruction, &mut map) {
                    continue;
                }
                assert!(map[next_pos.0][next_pos.1] == Tile::Empty);
                robot = next_pos;
            }
            Tile::Empty => {
                robot = next_pos;
            }
        }
        // print_map(&map, robot);
    }
    // print_map(&map, robot);
    let mut sum_of_box_coords = 0;
    for (i, row) in map.iter().enumerate() {
        for (j, tile) in row.iter().enumerate() {
            if *tile == Tile::BoxL {
                sum_of_box_coords += 100 * i as u64 + j as u64;
            }
        }
    }
    sum_of_box_coords.to_string()
}

fn push_boxes1(start: (usize, usize), dir: Direction, map: &mut [Vec<Tile>]) -> bool {
    let mut pos = start;
    match dir {
        Direction::Up => {
            pos.0 -= 1;
        }
        Direction::Down => {
            pos.0 += 1;
        }
        Direction::Left => {
            pos.1 -= 1;
        }
        Direction::Right => {
            pos.1 += 1;
        }
    }

    match map[pos.0][pos.1] {
        Tile::Wall => false,
        Tile::BoxL => {
            if !push_boxes1(pos, dir, map) {
                return false;
            }
            map[pos.0][pos.1] = map[start.0][start.1];
            map[start.0][start.1] = Tile::Empty;
            true
        }
        Tile::BoxR => panic!("BoxR"),
        Tile::Empty => {
            map[pos.0][pos.1] = map[start.0][start.1];
            map[start.0][start.1] = Tile::Empty;
            true
        }
    }
}

fn push_boxes2(start: (usize, usize), dir: Direction, map: &mut [Vec<Tile>]) -> bool {
    if map[start.0][start.1] == Tile::Empty {
        return true;
    }
    if map[start.0][start.1] == Tile::Wall {
        return false;
    }

    let mut pos = start;
    match dir {
        Direction::Up => {
            pos.0 -= 1;
        }
        Direction::Down => {
            pos.0 += 1;
        }
        Direction::Left => {
            pos.1 -= 1;
        }
        Direction::Right => {
            pos.1 += 1;
        }
    }
    if dir == Direction::Left || dir == Direction::Right {
        if !push_boxes2(pos, dir, map) {
            return false;
        }
        map[pos.0][pos.1] = map[start.0][start.1];
        map[start.0][start.1] = Tile::Empty;
        return true;
    }

    let pos2 = if map[start.0][start.1] == Tile::BoxL {
        (pos.0, pos.1 + 1)
    } else {
        (pos.0, pos.1 - 1)
    };

    if try_push_boxes(pos, dir, map) && try_push_boxes(pos2, dir, map) {
        assert!(push_boxes2(pos, dir, map) && push_boxes2(pos2, dir, map));
        if map[start.0][start.1] == Tile::BoxL {
            map[pos.0][pos.1] = Tile::BoxL;
            map[pos.0][pos.1 + 1] = Tile::BoxR;
            map[start.0][start.1] = Tile::Empty;
            map[start.0][start.1 + 1] = Tile::Empty;
        } else {
            map[pos.0][pos.1] = Tile::BoxR;
            map[pos.0][pos.1 - 1] = Tile::BoxL;
            map[start.0][start.1] = Tile::Empty;
            map[start.0][start.1 - 1] = Tile::Empty;
        };
        true
    } else {
        false
    }
}

// only for part 2
fn try_push_boxes(start: (usize, usize), dir: Direction, map: &[Vec<Tile>]) -> bool {
    if map[start.0][start.1] == Tile::Empty {
        return true;
    }
    if map[start.0][start.1] == Tile::Wall {
        return false;
    }
    assert!(matches!(map[start.0][start.1], Tile::BoxL | Tile::BoxR));

    let mut pos = start;
    match dir {
        Direction::Up => {
            pos.0 -= 1;
        }
        Direction::Down => {
            pos.0 += 1;
        }
        Direction::Left => {
            pos.1 -= 1;
        }
        Direction::Right => {
            pos.1 += 1;
        }
    }
    let pos2 = if map[start.0][start.1] == Tile::BoxL {
        (pos.0, pos.1 + 1)
    } else {
        (pos.0, pos.1 - 1)
    };
    match (map[pos.0][pos.1], map[pos2.0][pos2.1]) {
        (Tile::Wall, _) | (_, Tile::Wall) => false,
        (Tile::Empty, Tile::Empty) => true,
        (_, _) => try_push_boxes(pos, dir, map) && try_push_boxes(pos2, dir, map),
    }
}

#[allow(dead_code)]
fn print_map(map: &[Vec<Tile>], robot: (usize, usize)) {
    for (i, row) in map.iter().enumerate() {
        for (j, tile) in row.iter().enumerate() {
            if (i, j) == robot {
                print!("@");
            } else {
                match tile {
                    Tile::Wall => print!("#"),
                    Tile::BoxL => print!("["),
                    Tile::BoxR => print!("]"),
                    Tile::Empty => print!("."),
                }
            }
        }
        println!();
    }
}
