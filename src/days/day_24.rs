use itertools::Itertools;

use crate::input::{Input, Part};
use std::{collections::HashMap, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Gate {
    And,
    Or,
    Xor,
}

impl Gate {
    fn apply(&self, a: u8, b: u8) -> u8 {
        match self {
            Gate::And => a & b,
            Gate::Or => a | b,
            Gate::Xor => a ^ b,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy, Eq)]
enum State {
    True,
    False,
    Unknown,
}

impl FromStr for State {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(State::False),
            "1" => Ok(State::True),
            _ => Ok(State::Unknown),
        }
    }
}

impl FromStr for Gate {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "AND" => Ok(Gate::And),
            "OR" => Ok(Gate::Or),
            "XOR" => Ok(Gate::Xor),
            _ => Err(()),
        }
    }
}

impl From<State> for u8 {
    fn from(s: State) -> u8 {
        match s {
            State::True => 1,
            State::False => 0,
            State::Unknown => panic!(),
        }
    }
}

impl From<u8> for State {
    fn from(s: u8) -> State {
        match s {
            1 => State::True,
            0 => State::False,
            _ => State::Unknown,
        }
    }
}

pub(super) fn run(input: &Input, part: Part) -> String {
    let mut wires = HashMap::new();
    let mut gates = Vec::new();
    let mut lines = input.get().lines();
    for line in &mut lines {
        if line.is_empty() {
            break;
        }
        let (name, state) = line.split_once(": ").unwrap();
        wires.insert(name.to_string(), state.parse::<State>().unwrap());
    }
    for line in lines {
        let mut s = line.split(" ");
        let w1 = s.next().unwrap();
        let gate = s.next().unwrap();
        let w2 = s.next().unwrap();
        let _ = s.next();
        let w3 = s.next().unwrap();
        gates.push((
            w1.to_string(),
            gate.parse::<Gate>().unwrap(),
            w2.to_string(),
            w3.to_string(),
        ));
        // wires.insert(w1.to_string(), State::Unknown);
        // wires.insert(w2.to_string(), State::Unknown);
        wires.insert(w3.to_string(), State::Unknown);
    }
    if part == Part::One {
        while !gates.is_empty() {
            for i in 0..gates.len() {
                let g = &gates[i];
                let w1 = *wires.get(&g.0).unwrap();
                let w2 = *wires.get(&g.2).unwrap();
                if w1 != State::Unknown && w2 != State::Unknown {
                    let w3 = wires.get_mut(&g.3).unwrap();
                    *w3 = State::from(g.1.apply(w1.into(), w2.into()));
                    println!(
                        "saturated gate {:?}: {:?}({}) OP {:?}({}) = {:?}({})",
                        g.1, w1, g.0, w2, g.2, w3, g.3
                    );
                    gates.remove(i);
                    break;
                }
            }
        }

        let res: u64 = wires_to_int(
            wires
                .iter()
                .filter(|(k, _)| k.starts_with("z"))
                .map(|(k, v)| (k.replace("z", "").parse::<u8>().unwrap(), v)),
        );
        res.to_string()
    } else {
        let x_val = wires_to_int(
            wires
                .iter()
                .filter(|(k, _)| k.starts_with("x"))
                .map(|(k, v)| (k.replace("x", "").parse::<u8>().unwrap(), v)),
        );
        let y_val = wires_to_int(
            wires
                .iter()
                .filter(|(k, _)| k.starts_with("y"))
                .map(|(k, v)| (k.replace("y", "").parse::<u8>().unwrap(), v)),
        );

        // these changes were manually detected and are pre-applied
        // change 1: vss and z14
        // change 2: kdh and hjf
        for gate in gates.iter_mut() {
            if gate.3 == "z14" {
                gate.3 = "vss".to_string();
            } else if gate.3 == "vss" {
                gate.3 = "z14".to_string();
            }
            if gate.3 == "kdh" {
                gate.3 = "hjf".to_string();
            } else if gate.3 == "hjf" {
                gate.3 = "kdh".to_string();
            }
        }

        // these two changes are already automtically detected and applied
        // change 3: kpp and z31
        // change 4: z35 and sgj

        let changes = ["kpp", "z31", "z35", "sgj", "vss", "z14", "kdh", "hjf"];

        let graphviz = generate_graphviz(&gates);
        println!("{}", graphviz);

        let mut carry_one = vec!["INVALID".to_string()];
        let mut carry_two = vec!["INVALID".to_string()];
        let mut carry = vec![];
        let mut prelims = vec![];

        // first digit: no carry -> easy
        // x00 XOR y00 = z00
        // carry00 = x00 AND y00

        // second digit: only carry from first digit, no extended carry
        // prelim01 = x01 XOR y01
        // carryONE01 = x01 AND y01
        // carryTWO01 = prelim01 AND carry00
        // z01 = prelim01 XOR carry00
        // carry01 = carryONE01 OR carryTWO01

        for i in 0..45 {
            let x_label = format!("x{:02}", i);
            let y_label = format!("y{:02}", i);
            let xor = gates
                .iter()
                .find(|(w1, g, w2, _)| {
                    (*w1 == x_label && *g == Gate::Xor && *w2 == y_label)
                        || (*w1 == y_label && *g == Gate::Xor && *w2 == x_label)
                })
                .unwrap_or_else(|| {
                    panic!("missing XOR gate for x: {} and y: {}", x_label, y_label)
                });
            prelims.push(xor.3.clone());

            let and = gates
                .iter()
                .find(|(w1, g, w2, _)| {
                    (*w1 == x_label && *g == Gate::And && *w2 == y_label)
                        || (*w1 == y_label && *g == Gate::And && *w2 == x_label)
                })
                .unwrap();
            if i == 0 {
                carry.push(and.3.clone());
            } else {
                carry_one.push(and.3.clone());
            }
        }
        for (i, carry_one) in carry_one.iter().enumerate() {
            if i > 0 {
                let carry_prev = carry[i - 1].clone();
                let and = gates
                    .iter()
                    .find(|(w1, g, w2, _)| {
                        (*w1 == carry_prev && *g == Gate::And && *w2 == prelims[i])
                            || (*w1 == prelims[i] && *g == Gate::And && *w2 == carry_prev)
                    })
                    .unwrap_or_else(|| {
                        panic!(
                            "missing AND gate for carry_two{}:  carry= {} and prelim= {}",
                            i, carry_prev, prelims[i]
                        )
                    });
                let carry_two_value = and.3.clone();
                carry_two.push(carry_two_value.clone());

                let or = gates
                    .iter()
                    .find(|(w1, g, w2, _)| {
                        (w1 == carry_one && *g == Gate::Or && *w2 == carry_two_value)
                            || (*w1 == carry_two_value && *g == Gate::Or && w2 == carry_one)
                    })
                    .cloned();
                if or.is_none() {
                    println!(
                        "missing OR gate for carry_one{}:  carry= {} and and= {}",
                        i, carry_one, carry_two_value
                    );
                    // is carry_one input to an or gate?
                    let carry_one_or = gates.iter().find(|(w1, g, w2, _)| {
                        (w1 == carry_one && *g == Gate::Or) || (*g == Gate::Or && w2 == carry_one)
                    });
                    let output = if let Some(g) = carry_one_or.cloned() {
                        let invalid_wire = if g.0 == *carry_one {
                            g.2.clone()
                        } else {
                            g.0.clone()
                        };
                        println!(
                            "found wrong wire! changing {} and {}",
                            carry_two_value, invalid_wire
                        );
                        for gate in gates.iter_mut() {
                            if gate.3 == invalid_wire {
                                gate.3 = carry_two_value.clone();
                            } else if gate.3 == carry_two_value {
                                gate.3 = invalid_wire.clone();
                            }
                        }
                        g.3.clone()
                    } else {
                        // is carry_two input to an or gate?
                        let g = gates
                            .iter()
                            .find(|(w1, g, w2, _)| {
                                (*w1 == carry_two_value && *g == Gate::Or)
                                    || (*g == Gate::Or && *w2 == carry_two_value)
                            })
                            .unwrap()
                            .clone();
                        let invalid_wire = if g.0 == carry_two_value {
                            g.2.clone()
                        } else {
                            g.0.clone()
                        };
                        println!(
                            "found wrong wire! changing {} and {}",
                            carry_one, invalid_wire
                        );
                        for gate in gates.iter_mut() {
                            if gate.3 == invalid_wire {
                                gate.3 = carry_one.clone();
                            } else if gate.3 == *carry_one {
                                gate.3 = invalid_wire.clone();
                            }
                        }
                        g.3.clone()
                    };
                    carry.push(output);
                } else {
                    let or = or.unwrap();
                    carry.push(or.3.clone());
                }
            }
        }

        let mut outs = vec![];
        for i in 0..45 {
            let z_label = format!("z{:02}", i);
            if i == 0 {
                outs.push(prelims[i].clone());
            } else {
                let xor = gates
                    .iter()
                    .find(|(w1, g, w2, _)| {
                        (*w1 == prelims[i] && *g == Gate::Xor && *w2 == carry[i - 1])
                            || (*w1 == carry[i - 1] && *g == Gate::Xor && *w2 == prelims[i])
                    })
                    .unwrap_or_else(|| {
                        panic!(
                            "missing XOR gate for prelim: {} and carry: {}",
                            prelims[i],
                            carry[i - 1]
                        )
                    });
                outs.push(xor.3.clone());
            }
            assert!(outs[i] == z_label, "expected z{} but got {}", i, outs[i]);
        }

        while !gates.is_empty() {
            for i in 0..gates.len() {
                let g = &gates[i];
                let w1 = *wires.get(&g.0).unwrap();
                let w2 = *wires.get(&g.2).unwrap();
                if w1 != State::Unknown && w2 != State::Unknown {
                    let w3 = wires.get_mut(&g.3).unwrap();
                    *w3 = State::from(g.1.apply(w1.into(), w2.into()));
                    println!(
                        "saturated gate {:?}: {:?}({}) OP {:?}({}) = {:?}({})",
                        g.1, w1, g.0, w2, g.2, w3, g.3
                    );
                    gates.remove(i);
                    break;
                }
            }
        }

        let res: u64 = wires_to_int(
            wires
                .iter()
                .filter(|(k, _)| k.starts_with("z"))
                .map(|(k, v)| (k.replace("z", "").parse::<u8>().unwrap(), v)),
        );
        assert!(
            res == x_val + y_val,
            "expected {} but got {}",
            x_val + y_val,
            res
        );
        changes.to_vec().iter().sorted().join(",")
    }
}

fn wires_to_int<'a>(wires: impl Iterator<Item = (u8, &'a State)>) -> u64 {
    wires
        .sorted_by_key(|(k, _)| *k)
        .rev()
        .map(|(_, v)| {
            println!("{:?}", v);
            v
        })
        .fold(0, |acc, v| {
            (acc << 1)
                | match v {
                    State::True => 1,
                    State::False => 0,
                    _ => panic!(),
                }
        })
}

fn generate_graphviz(gates: &[(String, Gate, String, String)]) -> String {
    let mut graph = String::from("digraph G {\n");

    for (input1, op, input2, output) in gates {
        graph.push_str(&format!(
            "    \"{}\" -> \"{}\" [label=\"{:?} {} {}\"];\n",
            input1, output, op, input1, input2
        ));
        graph.push_str(&format!(
            "    \"{}\" -> \"{}\" [label=\"{:?} {} {}\"];\n",
            input2, output, op, input1, input2
        ));
    }

    graph.push_str("}\n");
    graph
}
