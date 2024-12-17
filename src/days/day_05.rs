use crate::input::{Input, Part};
use std::collections::{HashMap, HashSet};

pub(super) fn run(input: &Input, part: Part) -> String {
    if part == Part::One {
        let s = input.get();
        let mut found_sep = false;
        let mut rules = HashMap::new();
        let mut sum = 0;
        for line in s.lines() {
            if !found_sep {
                if line.is_empty() {
                    found_sep = true;
                    continue;
                }
                let (p1, p2) = line.split_once("|").unwrap();
                rules.entry(p2).or_insert(HashSet::new()).insert(p1);
            } else {
                let mut line_new = HashMap::new();
                for (i, l) in line.split(",").enumerate() {
                    line_new.insert(l, i);
                }
                let mut compliant = true;
                for (i, n) in line.split(",").enumerate() {
                    if rules.contains_key(n) {
                        for dep in rules.get(n).unwrap() {
                            if let Some(j) = line_new.get(dep) {
                                if *j >= i {
                                    compliant = false;
                                    break;
                                }
                            }
                        }
                    }
                    if !compliant {
                        break;
                    }
                }
                if compliant {
                    sum += line
                        .split(",")
                        .nth(line.split(",").count() / 2)
                        .unwrap()
                        .parse::<u64>()
                        .unwrap();
                }
            }
        }
        sum.to_string()
    } else {
        let s = input.get();
        let mut found_sep = false;
        let mut rules = HashMap::new();
        let mut updates = Vec::new();
        let mut sum = 0;
        for line in s.lines() {
            if !found_sep {
                if line.is_empty() {
                    found_sep = true;
                    continue;
                }
                let (p1, p2) = line.split_once("|").unwrap();
                rules.entry(p2).or_insert(HashSet::new()).insert(p1);
            } else {
                let line_split = line.split(",").collect::<Vec<_>>();
                updates.push(line_split);
            }
        }

        for up in updates {
            let top_sorting =
                get_top_sorting(rules.clone(), up.iter().map(|x| x.to_string()).collect());
            let mut has_problem = false;
            for i in 0..up.len() - 1 {
                if top_sorting.iter().position(|x| x == up[i])
                    > top_sorting.iter().position(|x| x == up[i + 1])
                {
                    has_problem = true;
                }
            }
            if has_problem {
                let mut new_up = Vec::new();
                for t in &top_sorting {
                    if up.contains(&t.as_str()) {
                        new_up.push(t);
                    }
                }
                sum += new_up
                    .iter()
                    .map(|x| x.parse::<u64>().unwrap())
                    .nth(up.len() / 2)
                    .unwrap();
            }
        }
        sum.to_string()
    }
}

fn get_top_sorting(
    mut rules: HashMap<&str, HashSet<&str>>,
    considered_nums: Vec<String>,
) -> Vec<String> {
    // clean rules
    let keys_to_remove = rules
        .keys()
        .filter(|x| !considered_nums.contains(&x.to_string()))
        .cloned()
        .collect::<Vec<_>>();
    for k in keys_to_remove {
        rules.remove(k);
    }
    for (_, v) in rules.iter_mut() {
        let values_to_remove = v
            .clone()
            .into_iter()
            .filter(|x| !considered_nums.contains(&x.to_string()));
        for e in values_to_remove {
            v.remove(e);
        }
    }

    // init top sorting
    let mut top_sorting = Vec::new();
    for n in &considered_nums {
        if !rules.contains_key(&n.as_str()) {
            top_sorting.push(n.clone());
        }
    }
    while !rules.is_empty() {
        let mut has_empty = None;
        for (k, v) in rules.iter() {
            if v.is_empty() {
                has_empty = Some(k.to_owned().to_string());
                break;
            }
            if v.iter().all(|x| top_sorting.contains(&x.to_string())) {
                has_empty = Some(k.to_owned().to_string());
                break;
            }
        }
        let has_empty = has_empty.unwrap();
        if !top_sorting.contains(&has_empty) {
            top_sorting.push(has_empty.clone());
        }
        for rule in rules.iter_mut() {
            if rule.1.contains(has_empty.as_str()) {
                rule.1.remove(has_empty.as_str());
            }
        }
        rules.remove(has_empty.as_str());
    }
    top_sorting
}
