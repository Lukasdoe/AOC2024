use crate::input::{Input, Part};
use std::collections::HashMap;

pub(super) fn run(input: &Input, part: Part) -> String {
    let mut map = Vec::new();
    for line in input.get().lines() {
        map.push(
            line.chars()
                .map(|c| (c, None))
                .collect::<Vec<(char, Option<u64>)>>(),
        );
    }

    let mut next_region_id = 0;
    let mut regions = HashMap::new();
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            let c = map[y][x].0;
            let mut borders = 0;
            if (y == 0 || map[y - 1][x].0 != c)
                && ((part == Part::One)
                    || (x == 0 || map[y][x - 1].0 != c || (y != 0 && map[y - 1][x - 1].0 == c)))
            {
                borders += 1;
            }
            if (x == 0 || map[y][x - 1].0 != c)
                && ((part == Part::One)
                    || (y == 0 || map[y - 1][x].0 != c || (x != 0 && map[y - 1][x - 1].0 == c)))
            {
                borders += 1;
            }
            if (x == map[0].len() - 1 || map[y][x + 1].0 != c)
                && ((part == Part::One)
                    || (y == 0
                        || map[y - 1][x].0 != c
                        || (x != map[0].len() - 1 && map[y - 1][x + 1].0 == c)))
            {
                borders += 1;
            }
            if (y == map.len() - 1 || map[y + 1][x].0 != c)
                && ((part == Part::One)
                    || (x == 0
                        || map[y][x - 1].0 != c
                        || (y != map.len() - 1 && map[y + 1][x - 1].0 == c)))
            {
                borders += 1;
            }

            let upper_region = y != 0 && map[y - 1][x].0 == c;
            let left_region = x != 0 && map[y][x - 1].0 == c;
            let region_id = match (upper_region, left_region) {
                (true, true) => {
                    let region_id_1 = map[y - 1][x].1.unwrap();
                    let region_id_2 = map[y][x - 1].1.unwrap();
                    if region_id_1 == region_id_2 {
                        region_id_1
                    } else {
                        merge_regions(&mut map, &mut regions, region_id_1, region_id_2)
                    }
                }
                (true, false) => map[y - 1][x].1.unwrap(),
                (false, true) => map[y][x - 1].1.unwrap(),
                (false, false) => {
                    next_region_id += 1;
                    regions.insert(next_region_id, (0, 0));
                    next_region_id
                }
            };
            map[y][x].1 = Some(region_id);
            regions.get_mut(&region_id).unwrap().0 += 1;
            regions.get_mut(&region_id).unwrap().1 += borders;
        }
    }
    // for i in 0..map.len() {
    //     for j in 0..map[0].len() {
    //         print!(" {}|{:2} ", map[i][j].0, map[i][j].1.unwrap());
    //     }
    //     println!();
    // }

    regions
        .values()
        .map(|(area, bs)| area * bs)
        .sum::<u64>()
        .to_string()
}

fn merge_regions(
    region_map: &mut [Vec<(char, Option<u64>)>],
    regions: &mut HashMap<u64, (u64, u64)>,
    region_id_1: u64,
    region_id_2: u64,
) -> u64 {
    for row in region_map.iter_mut() {
        for (_, region_id) in row.iter_mut() {
            if region_id.is_some() && region_id.unwrap() == region_id_2 {
                *region_id = Some(region_id_1);
            }
        }
    }
    let (area, bs) = regions.remove(&region_id_2).unwrap();
    regions.get_mut(&region_id_1).unwrap().0 += area;
    regions.get_mut(&region_id_1).unwrap().1 += bs;
    region_id_1
}
