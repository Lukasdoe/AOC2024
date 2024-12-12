use crate::input::{Input, Part};

pub(super) fn run(input: &Input, part: Part) -> u64 {
    let s = input.get();
    assert!(s.lines().count() == 1);
    let line = s.lines().next().unwrap();
    let mut blocks = Vec::new();
    let mut spaces = Vec::new();
    line.chars().enumerate().for_each(|(i, c)| {
        if i % 2 == 0 {
            blocks.push(c.to_digit(10).unwrap() as u64);
        } else {
            spaces.push(c.to_digit(10).unwrap() as u64);
        }
    });

    if part == Part::One {
        let mut checksum = 0;
        let mut i = 0;
        let mut j = 0;
        let mut blocks_start = 0;
        let mut blocks_end = blocks.len() - 1;
        while i < blocks.len() + spaces.len() {
            if i % 2 == 0 {
                let b = blocks[blocks_start];
                for k in 0..b {
                    checksum += (j + k) * blocks_start as u64;
                }
                blocks_start += 1;
                j += b;
            } else {
                let space = spaces.remove(0);
                for k in 0..space {
                    while blocks[blocks_end] == 0 {
                        blocks_end -= 1;
                    }
                    if blocks_end < blocks_start {
                        break;
                    }
                    blocks[blocks_end] -= 1;
                    checksum += (j + k) * blocks_end as u64;
                }
                j += space;
            }
            i += 1;
        }
        checksum
    } else {
        enum State {
            Block((u64, u64)),
            Space(u64),
        }
        let mut fs = Vec::new();
        for i in 0..blocks.len() {
            fs.push(State::Block((i as u64, blocks[i])));
            if i < spaces.len() {
                fs.push(State::Space(spaces[i]));
            }
        }
        while let Some(b) = blocks.pop() {
            for i in 0..fs.len() {
                match fs[i] {
                    State::Space(n) if n >= b => {
                        fs[i] = State::Space(n - b);
                        fs.insert(i, State::Block((blocks.len() as u64, b)));
                        let idx = fs
                            .iter()
                            .enumerate()
                            .rev()
                            .find(|(_, x)| match x {
                                State::Block((id, _)) => *id == blocks.len() as u64,
                                _ => false,
                            })
                            .unwrap()
                            .0;
                        fs[idx] = State::Space(b);
                        break;
                    }
                    _ => (),
                }
            }
        }
        // for i in &fs {
        //     match i {
        //         State::Block((id, b)) => {
        //             for _ in 0..*b {
        //                 print!("{}", id);
        //             }
        //         }
        //         State::Space(n) => {
        //             for _ in 0..*n {
        //                 print!(".");
        //             }
        //         }
        //     }
        // }
        // println!();

        let mut j = 0;
        let mut checksum = 0;
        for item in fs {
            match item {
                State::Block((i, b)) => {
                    for k in 0..b {
                        checksum += (j + k) * i;
                    }
                    j += b;
                }
                State::Space(n) => {
                    j += n;
                }
            }
        }
        checksum
    }
}
