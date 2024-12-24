fn main() {
    let input: &str = &include_str!("../data/24.txt").replace("\r", "");
    println!("Answer to part1: {}", part1(input));
    println!("Answer to part2: {}", part2(input));
}

use std::collections::HashMap;

#[derive(Clone)]
enum Formula {
    And(String, String),
    Or(String, String),
    Xor(String, String),
}

fn parse_input(input: &str) -> (HashMap<String, u64>, HashMap<String, Formula>) {
    let mut known = HashMap::new();
    let mut formulas = HashMap::new();

    let (initial_values, formula_lines) = input.split_once("\n\n").unwrap();
    // Parse initial values
    for line in initial_values.lines() {
        let (wire, value) = line.split_once(": ").unwrap();
        known.insert(wire.to_string(), value.parse().unwrap());
    }

    // Parse formulas
    for line in formula_lines.lines() {
        let parts: Vec<_> = line.split_whitespace().collect();
        let (x, op, y, z) = (parts[0], parts[1], parts[2], parts[4]);
        let formula = match op {
            "AND" => Formula::And(x.to_string(), y.to_string()),
            "OR" => Formula::Or(x.to_string(), y.to_string()),
            "XOR" => Formula::Xor(x.to_string(), y.to_string()),
            _ => panic!("Unknown operator"),
        };
        formulas.insert(z.to_string(), formula);
    }

    (known, formulas)
}

fn calc(wire: &str, known: &mut HashMap<String, u64>, formulas: &HashMap<String, Formula>) -> u64 {
    if let Some(&value) = known.get(wire) {
        return value;
    }

    let formula = formulas.get(wire).unwrap();
    let result = match formula {
        Formula::And(x, y) => calc(x, known, formulas) & calc(y, known, formulas),
        Formula::Or(x, y) => calc(x, known, formulas) | calc(y, known, formulas),
        Formula::Xor(x, y) => calc(x, known, formulas) ^ calc(y, known, formulas),
    };

    known.insert(wire.to_string(), result);
    result
}

fn part1(input: &str) -> u64 {
    let (mut known, formulas) = parse_input(input);

    let mut result = 0;
    for i in 0.. {
        let wire = format!("z{:02}", i);
        if !formulas.contains_key(&wire) {
            break;
        }
        result |= calc(&wire, &mut known, &formulas) << i;
    }
    result
}

fn make_wire(char: &str, num: usize) -> String {
    format!("{}{:02}", char, num)
}

fn verify_z(formulas: &HashMap<String, Formula>, wire: &str, num: usize) -> bool {
    if let Some(Formula::Xor(x, y)) = formulas.get(wire) {
        if num == 0 {
            let mut inputs = vec![x.as_str(), y.as_str()];
            inputs.sort();
            return inputs == ["x00", "y00"];
        }
        (verify_intermediate_xor(formulas, x, num) && verify_carry_bit(formulas, y, num))
            || (verify_intermediate_xor(formulas, y, num) && verify_carry_bit(formulas, x, num))
    } else {
        false
    }
}

fn verify_intermediate_xor(formulas: &HashMap<String, Formula>, wire: &str, num: usize) -> bool {
    if let Some(Formula::Xor(x, y)) = formulas.get(wire) {
        let mut inputs = vec![x.as_str(), y.as_str()];
        inputs.sort();
        inputs == [&make_wire("x", num), &make_wire("y", num)]
    } else {
        false
    }
}

fn verify_carry_bit(formulas: &HashMap<String, Formula>, wire: &str, num: usize) -> bool {
    if let Some(formula) = formulas.get(wire) {
        match formula {
            Formula::And(x, y) if num == 1 => {
                let mut inputs = vec![x.as_str(), y.as_str()];
                inputs.sort();
                inputs == ["x00", "y00"]
            }
            Formula::Or(x, y) => {
                (verify_direct_carry(formulas, x, num - 1) && verify_recarry(formulas, y, num - 1))
                    || (verify_direct_carry(formulas, y, num - 1)
                        && verify_recarry(formulas, x, num - 1))
            }
            _ => false,
        }
    } else {
        false
    }
}

fn verify_direct_carry(formulas: &HashMap<String, Formula>, wire: &str, num: usize) -> bool {
    if let Some(Formula::And(x, y)) = formulas.get(wire) {
        let mut inputs = vec![x.as_str(), y.as_str()];
        inputs.sort();
        inputs == [&make_wire("x", num), &make_wire("y", num)]
    } else {
        false
    }
}

fn verify_recarry(formulas: &HashMap<String, Formula>, wire: &str, num: usize) -> bool {
    if let Some(Formula::And(x, y)) = formulas.get(wire) {
        (verify_intermediate_xor(formulas, x, num) && verify_carry_bit(formulas, y, num))
            || (verify_intermediate_xor(formulas, y, num) && verify_carry_bit(formulas, x, num))
    } else {
        false
    }
}

fn verify(formulas: &HashMap<String, Formula>, num: usize) -> bool {
    verify_z(formulas, &make_wire("z", num), num)
}

fn progress(formulas: &HashMap<String, Formula>) -> usize {
    let mut i = 0;
    while verify(formulas, i) {
        i += 1;
    }
    i
}

fn swap_wires(formulas: &mut HashMap<String, Formula>, wire1: &str, wire2: &str) {
    if let (Some(formula1), Some(formula2)) = (formulas.remove(wire1), formulas.remove(wire2)) {
        formulas.insert(wire1.to_string(), formula2);
        formulas.insert(wire2.to_string(), formula1);
    }
}

fn part2(input: &str) -> String {
    let (_, mut formulas) = parse_input(input);
    let mut swaps = Vec::new();

    for _ in 0..4 {
        let baseline = progress(&formulas);
        let mut found = false;
        let keys: Vec<String> = formulas.keys().cloned().collect();
        for wire1 in keys.iter() {
            if found {
                break;
            }

            for wire2 in keys.iter() {
                if wire1 == wire2 {
                    continue;
                }

                let wire1 = wire1.to_string();
                let wire2 = wire2.to_string();

                swap_wires(&mut formulas, &wire1, &wire2);
                if progress(&formulas) > baseline {
                    swaps.push(wire1);
                    swaps.push(wire2);
                    found = true;
                    break;
                }
                swap_wires(&mut formulas, &wire1, &wire2);
            }
        }
        if !found {
            return "No improvement found".to_string();
        }
    }

    swaps.sort();
    swaps.join(",")
}

#[test]
fn test_part1() {
    assert_eq!(
        part1(&include_str!("../data/test/24.txt").replace("\r", "")),
        9
    );
}

#[test]
fn test_part2() {
    assert_eq!(
        part2(&include_str!("../data/test/24.txt").replace("\r", "")),
        "No improvement found"
    );
}
