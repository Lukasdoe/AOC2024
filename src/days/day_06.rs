use crate::input::{Input, Part};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    Empty,
    Wall,
    Obstacle,
    Visited {
        up: bool,
        down: bool,
        left: bool,
        right: bool,
    },
}

impl Tile {
    fn is_visited(&self) -> bool {
        matches!(self, Tile::Visited { .. })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub(super) fn run(input: &Input, part: Part) -> u64 {
    let mut map = Vec::new();
    let mut guard_x = 0;
    let mut guard_y = 0;
    let mut guard_dir = Direction::Up;

    for line in input.get().lines() {
        let mut row = Vec::new();
        row.push(Tile::Wall);
        for c in line.chars() {
            match c {
                '.' => row.push(Tile::Empty),
                '#' => row.push(Tile::Obstacle),
                '^' => {
                    guard_x = row.len();
                    guard_y = map.len();
                    guard_dir = Direction::Up;
                    row.push(Tile::Empty);
                }
                'v' => {
                    guard_x = row.len();
                    guard_y = map.len();
                    guard_dir = Direction::Down;
                    row.push(Tile::Empty);
                }
                '<' => {
                    guard_x = row.len();
                    guard_y = map.len();
                    guard_dir = Direction::Left;
                    row.push(Tile::Empty);
                }
                '>' => {
                    guard_x = row.len();
                    guard_y = map.len();
                    guard_dir = Direction::Right;
                    row.push(Tile::Empty);
                }
                _ => unreachable!(),
            };
        }
        row.push(Tile::Wall);
        map.push(row);
    }
    map.insert(0, vec![Tile::Wall; map[0].len()]);
    map.push(vec![Tile::Wall; map[0].len()]);

    // print_map(&map);

    if part == Part::One {
        for _ in 0..(map.len() * map[0].len()) {
            map[guard_y][guard_x] = match guard_dir {
                Direction::Up => Tile::Visited {
                    up: true,
                    down: false,
                    left: false,
                    right: false,
                },
                Direction::Down => Tile::Visited {
                    up: false,
                    down: true,
                    left: false,
                    right: false,
                },
                Direction::Left => Tile::Visited {
                    up: false,
                    down: false,
                    left: true,
                    right: false,
                },
                Direction::Right => Tile::Visited {
                    up: false,
                    down: false,
                    left: false,
                    right: true,
                },
            };

            let (new_x, new_y) = match guard_dir {
                Direction::Up => (guard_x, guard_y - 1),
                Direction::Down => (guard_x, guard_y + 1),
                Direction::Left => (guard_x - 1, guard_y),
                Direction::Right => (guard_x + 1, guard_y),
            };
            if map[new_y][new_x] == Tile::Wall {
                break;
            }
            if map[new_y][new_x] == Tile::Obstacle {
                guard_dir = match guard_dir {
                    Direction::Up => Direction::Right,
                    Direction::Right => Direction::Down,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Up,
                }
            } else {
                guard_x = new_x;
                guard_y = new_y;
            }
        }

        // print_map(&map);
        map.iter()
            .map(|row| row.iter().filter(|t| t.is_visited()).count() as u64)
            .sum::<u64>()
    } else {
        let mut possible_obstacles = 0;
        let mut possible_obstacles_pos = Vec::new();
        for y in 0..map.len() {
            for x in 0..map[0].len() {
                if map[y][x] == Tile::Empty {
                    let mut new_map = map.clone();
                    new_map[y][x] = Tile::Obstacle;
                    if map_has_loop(new_map, guard_x, guard_y, guard_dir) {
                        possible_obstacles += 1;
                        possible_obstacles_pos.push((x, y));
                    }
                }
            }
        }
        // print_map_with_obstacles(&map, &possible_obstacles_pos);
        possible_obstacles
    }
}

// fn print_map(map: &Vec<Vec<Tile>>) {
//     for row in map {
//         for tile in row {
//             match tile {
//                 Tile::Empty => print!("."),
//                 Tile::Wall => print!("!"),
//                 Tile::Obstacle => print!("#"),
//                 Tile::Visited {
//                     up,
//                     down,
//                     left,
//                     right,
//                 } => {
//                     if *up {
//                         print!("^");
//                     } else if *down {
//                         print!("v");
//                     } else if *left {
//                         print!("<");
//                     } else if *right {
//                         print!(">");
//                     } else {
//                         print!("?");
//                     }
//                 }
//             }
//         }
//         println!();
//     }
// }

// fn print_map_with_obstacles(map: &[Vec<Tile>], obstacles: &[(usize, usize)]) {
//     for (y, row) in map.iter().enumerate() {
//         for (x, tile) in row.iter().enumerate() {
//             if obstacles.iter().any(|(ox, oy)| *ox == x && *oy == y) {
//                 print!("O");
//             } else {
//                 match tile {
//                     Tile::Empty => print!("."),
//                     Tile::Wall => print!("!"),
//                     Tile::Obstacle => print!("#"),
//                     Tile::Visited {
//                         up,
//                         down,
//                         left,
//                         right,
//                     } => {
//                         if *up {
//                             print!("^");
//                         } else if *down {
//                             print!("v");
//                         } else if *left {
//                             print!("<");
//                         } else if *right {
//                             print!(">");
//                         } else {
//                             print!("?");
//                         }
//                     }
//                 }
//             }
//         }
//         println!();
//     }
// }

fn map_has_loop(
    mut map: Vec<Vec<Tile>>,
    mut guard_x: usize,
    mut guard_y: usize,
    mut guard_dir: Direction,
) -> bool {
    for _ in 0..(map.len() * map[0].len()) {
        map[guard_y][guard_x] = if map[guard_y][guard_x].is_visited() {
            match map[guard_y][guard_x] {
                Tile::Visited {
                    up,
                    down,
                    left,
                    right,
                } => {
                    match guard_dir {
                        Direction::Up if up => {
                            return true;
                        }
                        Direction::Down if down => {
                            return true;
                        }
                        Direction::Left if left => {
                            return true;
                        }
                        Direction::Right if right => {
                            return true;
                        }
                        _ => {}
                    }

                    Tile::Visited {
                        up: up || guard_dir == Direction::Up,
                        down: down || guard_dir == Direction::Down,
                        left: left || guard_dir == Direction::Left,
                        right: right || guard_dir == Direction::Right,
                    }
                }
                _ => unreachable!(),
            }
        } else {
            match guard_dir {
                Direction::Up => Tile::Visited {
                    up: true,
                    down: false,
                    left: false,
                    right: false,
                },
                Direction::Down => Tile::Visited {
                    up: false,
                    down: true,
                    left: false,
                    right: false,
                },
                Direction::Left => Tile::Visited {
                    up: false,
                    down: false,
                    left: true,
                    right: false,
                },
                Direction::Right => Tile::Visited {
                    up: false,
                    down: false,
                    left: false,
                    right: true,
                },
            }
        };
        let (new_x, new_y) = match guard_dir {
            Direction::Up => (guard_x, guard_y - 1),
            Direction::Down => (guard_x, guard_y + 1),
            Direction::Left => (guard_x - 1, guard_y),
            Direction::Right => (guard_x + 1, guard_y),
        };
        if map[new_y][new_x] == Tile::Wall {
            break;
        }
        if map[new_y][new_x] == Tile::Obstacle {
            guard_dir = match guard_dir {
                Direction::Up => Direction::Right,
                Direction::Right => Direction::Down,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
            }
        } else {
            guard_x = new_x;
            guard_y = new_y;
        }
    }
    false
}
