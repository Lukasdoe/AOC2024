use crate::input::{Input, Part};

pub(super) fn run(input: &Input, part: Part) -> String {
    let mut m = Vec::new();
    for line in input.get().lines() {
        m.push(line.chars().collect::<Vec<_>>());
    }
    m.insert(0, vec!['.'; m[0].len()]);
    m.insert(0, vec!['.'; m[0].len()]);
    m.insert(0, vec!['.'; m[0].len()]);
    m.push(vec!['.'; m[0].len()]);
    m.push(vec!['.'; m[0].len()]);
    m.push(vec!['.'; m[0].len()]);
    for line in &mut m {
        line.insert(0, '.');
        line.insert(0, '.');
        line.insert(0, '.');
        line.push('.');
        line.push('.');
        line.push('.');
    }
    if part == Part::One {
        let mut cnt = 0;
        for i in 0..m.len() {
            for j in 0..m[i].len() {
                if m[i][j] == 'X' && m[i + 1][j] == 'M' && m[i + 2][j] == 'A' && m[i + 3][j] == 'S'
                {
                    cnt += 1;
                }
                if m[i][j] == 'S' && m[i + 1][j] == 'A' && m[i + 2][j] == 'M' && m[i + 3][j] == 'X'
                {
                    cnt += 1;
                }
                if m[i][j] == 'X' && m[i][j + 1] == 'M' && m[i][j + 2] == 'A' && m[i][j + 3] == 'S'
                {
                    cnt += 1;
                }
                if m[i][j] == 'S' && m[i][j + 1] == 'A' && m[i][j + 2] == 'M' && m[i][j + 3] == 'X'
                {
                    cnt += 1;
                }
                if m[i][j] == 'X'
                    && m[i + 1][j + 1] == 'M'
                    && m[i + 2][j + 2] == 'A'
                    && m[i + 3][j + 3] == 'S'
                {
                    cnt += 1;
                }
                if m[i][j] == 'S'
                    && m[i + 1][j + 1] == 'A'
                    && m[i + 2][j + 2] == 'M'
                    && m[i + 3][j + 3] == 'X'
                {
                    cnt += 1;
                }
                if m[i][j] == 'X'
                    && m[i + 1][j - 1] == 'M'
                    && m[i + 2][j - 2] == 'A'
                    && m[i + 3][j - 3] == 'S'
                {
                    cnt += 1;
                }
                if m[i][j] == 'S'
                    && m[i + 1][j - 1] == 'A'
                    && m[i + 2][j - 2] == 'M'
                    && m[i + 3][j - 3] == 'X'
                {
                    cnt += 1;
                }
            }
        }
        cnt.to_string()
    } else {
        let mut cnt = 0;
        for i in 0..m.len() {
            for j in 0..m[i].len() {
                if m[i][j] != 'A' {
                    continue;
                }
                if !(m[i + 1][j + 1] == 'M' && m[i - 1][j - 1] == 'S'
                    || m[i + 1][j + 1] == 'S' && m[i - 1][j - 1] == 'M')
                {
                    continue;
                }
                if !(m[i + 1][j - 1] == 'M' && m[i - 1][j + 1] == 'S'
                    || m[i + 1][j - 1] == 'S' && m[i - 1][j + 1] == 'M')
                {
                    continue;
                }
                cnt += 1;
            }
        }
        cnt.to_string()
    }
}
