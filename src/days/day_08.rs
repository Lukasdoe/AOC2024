use crate::input::{Input, Part};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Node(char),
}

pub(super) fn run(input: &Input, part: Part) -> String {
    let mut map = Vec::new();
    for line in input.get().lines() {
        map.push(
            line.chars()
                .map(|c| match c {
                    '.' => Tile::Empty,
                    c => Tile::Node(c),
                })
                .collect::<Vec<_>>(),
        );
    }
    let mut antinodes = (0..map.len())
        .map(|_| vec![false; map[0].len()])
        .collect::<Vec<_>>();
    for row in 0..map.len() {
        for col in 0..map[0].len() {
            if let Tile::Node(c) = map[row][col] {
                let mut radius = 1;
                loop {
                    let positions = get_circle_coords(row, col, radius, map.len(), map[0].len());
                    if positions.is_empty() {
                        break;
                    }
                    for p in positions
                        .iter()
                        .filter(|(x, y)| map[*x][*y] == Tile::Node(c))
                    {
                        if part == Part::One {
                            if let Some(coords) =
                                mirror_coords(p.0, p.1, row, col, map.len(), map[0].len()).first()
                            {
                                antinodes[coords.0][coords.1] = true;
                            }
                        } else {
                            for coords in mirror_coords(p.0, p.1, row, col, map.len(), map[0].len())
                            {
                                antinodes[coords.0][coords.1] = true;
                            }
                            antinodes[row][col] = true;
                        }
                    }
                    radius += 1;
                }
            }
        }
    }
    for row in 0..map.len() {
        for col in 0..map[0].len() {
            if antinodes[row][col] {
                map[row][col] = Tile::Node('#');
            }
        }
    }
    print_map(&map);
    antinodes
        .iter()
        .flatten()
        .filter(|&&b| b)
        .count()
        .to_string()
}

fn get_circle_coords(
    row: usize,
    col: usize,
    radius: usize,
    max_row: usize,
    max_col: usize,
) -> Vec<(usize, usize)> {
    let mut coords = Vec::new();
    for i in 0..=radius {
        if col + i < max_col {
            if row >= radius {
                coords.push((row - radius, col + i));
            }
            if row + radius < max_row {
                coords.push((row + radius, col + i));
            }
        }
        if row + i < max_row {
            if col >= radius {
                coords.push((row + i, col - radius));
            }
            if col + radius < max_col {
                coords.push((row + i, col + radius));
            }
        }

        if i != 0 {
            if col >= i {
                if row >= radius {
                    coords.push((row - radius, col - i));
                }
                if row + radius < max_row {
                    coords.push((row + radius, col - i));
                }
            }
            if row >= i {
                if col >= radius {
                    coords.push((row - i, col - radius));
                }
                if col + radius < max_col {
                    coords.push((row - i, col + radius));
                }
            }
        }
    }
    coords
}

fn print_map(map: &[Vec<Tile>]) {
    for row in map {
        for tile in row {
            match tile {
                Tile::Empty => print!("."),
                Tile::Node(c) => print!("{}", c),
            }
        }
        println!();
    }
}

fn mirror_coords(
    x: usize,
    y: usize,
    center_x: usize,
    center_y: usize,
    max_x: usize,
    max_y: usize,
) -> Vec<(usize, usize)> {
    let x = x as isize;
    let y = y as isize;
    let center_x = center_x as isize;
    let center_y = center_y as isize;

    let center_diff_x = center_x - x;
    let center_diff_y = center_y - y;

    let mut res = Vec::new();
    let mut new_x = center_x;
    let mut new_y = center_y;
    loop {
        new_x += center_diff_x;
        new_y += center_diff_y;
        if new_x < 0 || new_y < 0 {
            break;
        }
        if new_x >= max_x as isize || new_y >= max_y as isize {
            break;
        }
        res.push((new_x as usize, new_y as usize));
    }
    res
}
