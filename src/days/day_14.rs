use std::collections::HashSet;

use crate::input::{Input, Part};
use regex::Regex;

const WIDTH: i64 = 101;
const HEIGHT: i64 = 103;

// const DBG_WIDTH: i64 = 11;
// const DBG_HEIGHT: i64 = 7;

const SIMULATION_TIME: i64 = 100;

pub(super) fn run(input: &Input, part: Part) -> String {
    let width = WIDTH;
    let height = HEIGHT;

    if part == Part::One {
        let mut quad0 = 0;
        let mut quad1 = 0;
        let mut quad2 = 0;
        let mut quad3 = 0;

        let input_re = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();
        for line in input.get().lines() {
            let captures = input_re.captures(line).unwrap();
            let x = captures[1].parse::<i64>().unwrap();
            let y = captures[2].parse::<i64>().unwrap();
            let vx = captures[3].parse::<i64>().unwrap();
            let vy = captures[4].parse::<i64>().unwrap();
            let mut final_robot_x = (x + vx * SIMULATION_TIME) % width;
            let mut final_robot_y = (y + vy * SIMULATION_TIME) % height;
            if final_robot_x < 0 {
                final_robot_x += width;
            }
            if final_robot_y < 0 {
                final_robot_y += height;
            }
            match final_robot_x.cmp(&(width / 2)) {
                std::cmp::Ordering::Less => match final_robot_y.cmp(&(height / 2)) {
                    std::cmp::Ordering::Less => {
                        quad0 += 1;
                    }
                    std::cmp::Ordering::Greater => {
                        quad1 += 1;
                    }
                    _ => (),
                },
                std::cmp::Ordering::Greater => match final_robot_y.cmp(&(height / 2)) {
                    std::cmp::Ordering::Less => {
                        quad2 += 1;
                    }
                    std::cmp::Ordering::Greater => {
                        quad3 += 1;
                    }
                    _ => (),
                },
                _ => (),
            }
        }
        (quad0 * quad1 * quad2 * quad3).to_string()
    } else {
        let input_re = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();

        let mut robots = vec![];
        for line in input.get().lines() {
            let captures = input_re.captures(line).unwrap();
            let x = captures[1].parse::<i64>().unwrap();
            let y = captures[2].parse::<i64>().unwrap();
            let vx = captures[3].parse::<i64>().unwrap();
            let vy = captures[4].parse::<i64>().unwrap();
            robots.push((x, y, vx, vy));
        }
        let mut i = 0;
        for robot in robots.iter_mut() {
            let mut final_robot_x = (robot.0 + robot.2 * i) % width;
            let mut final_robot_y = (robot.1 + robot.3 * i) % height;
            if final_robot_x < 0 {
                final_robot_x += width;
            }
            if final_robot_y < 0 {
                final_robot_y += height;
            }
            robot.0 = final_robot_x;
            robot.1 = final_robot_y;
        }
        loop {
            let mut location_shared = false;
            let mut map = HashSet::new();
            for robot in robots.iter_mut() {
                let mut final_robot_x = (robot.0 + robot.2) % width;
                let mut final_robot_y = (robot.1 + robot.3) % height;
                if final_robot_x < 0 {
                    final_robot_x += width;
                }
                if final_robot_y < 0 {
                    final_robot_y += height;
                }
                if map.contains(&(final_robot_y, final_robot_x)) {
                    location_shared = true;
                }
                map.insert((final_robot_y, final_robot_x));
                robot.0 = final_robot_x;
                robot.1 = final_robot_y;
            }
            i += 1;
            if !location_shared {
                return i.to_string();
            }
        }
    }
}
