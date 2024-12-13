fn main() {
    let input: &str = &include_str!("../data/13.txt").replace("\r", "");
    println!("Answer to part1: {}", part1(input));
    println!("Answer to part2: {}", part2(input));
}

use std::collections::HashSet;

#[derive(Debug)]
struct ClawMachine {
    button_a: (i64, i64), // (X, Y) movement for button A
    button_b: (i64, i64), // (X, Y) movement for button B
    prize: (i64, i64),    // Prize location (X, Y)
}

fn parse_input(input: &str) -> Vec<ClawMachine> {
    let mut machines = Vec::new();
    let mut current_machine = None;

    for line in input.lines() {
        if line.trim().is_empty() {
            if let Some(machine) = current_machine.take() {
                machines.push(machine);
            }
            continue;
        }

        if line.starts_with("Button A:") {
            let parts: Vec<&str> = line.split(", ").collect();
            let x = parts[0].split("+").nth(1).unwrap().parse::<i64>().unwrap();
            let y = parts[1].split("+").nth(1).unwrap().parse::<i64>().unwrap();
            current_machine = Some(ClawMachine {
                button_a: (x, y),
                button_b: (0, 0),
                prize: (0, 0),
            });
        } else if line.starts_with("Button B:") {
            if let Some(ref mut machine) = current_machine {
                let parts: Vec<&str> = line.split(", ").collect();
                let x = parts[0].split("+").nth(1).unwrap().parse::<i64>().unwrap();
                let y = parts[1].split("+").nth(1).unwrap().parse::<i64>().unwrap();
                machine.button_b = (x, y);
            }
        } else if line.starts_with("Prize:") {
            if let Some(ref mut machine) = current_machine {
                let parts: Vec<&str> = line.split(", ").collect();
                let x = parts[0].split("=").nth(1).unwrap().parse::<i64>().unwrap();
                let y = parts[1].split("=").nth(1).unwrap().parse::<i64>().unwrap();
                machine.prize = (x, y);
            }
        }
    }

    if let Some(machine) = current_machine {
        machines.push(machine);
    }

    machines
}
fn parse_input_2(input: &str) -> Vec<ClawMachine> {
    let mut machines = Vec::new();
    let mut current_machine = None;

    for line in input.lines() {
        if line.trim().is_empty() {
            if let Some(machine) = current_machine.take() {
                machines.push(machine);
            }
            continue;
        }

        if line.starts_with("Button A:") {
            let parts: Vec<&str> = line.split(", ").collect();
            let x = parts[0].split("+").nth(1).unwrap().parse::<i64>().unwrap();
            let y = parts[1].split("+").nth(1).unwrap().parse::<i64>().unwrap();
            current_machine = Some(ClawMachine {
                button_a: (x, y),
                button_b: (0, 0),
                prize: (0, 0),
            });
        } else if line.starts_with("Button B:") {
            if let Some(ref mut machine) = current_machine {
                let parts: Vec<&str> = line.split(", ").collect();
                let x = parts[0].split("+").nth(1).unwrap().parse::<i64>().unwrap();
                let y = parts[1].split("+").nth(1).unwrap().parse::<i64>().unwrap();
                machine.button_b = (x, y);
            }
        } else if line.starts_with("Prize:") {
            if let Some(ref mut machine) = current_machine {
                let parts: Vec<&str> = line.split(", ").collect();
                let x =
                    parts[0].split("=").nth(1).unwrap().parse::<i64>().unwrap() + 10000000000000;
                let y =
                    parts[1].split("=").nth(1).unwrap().parse::<i64>().unwrap() + 10000000000000;
                machine.prize = (x, y);
            }
        }
    }

    if let Some(machine) = current_machine {
        machines.push(machine);
    }

    machines
}

fn solve_linear_equations(
    a1: i64,
    b1: i64,
    c1: i64,
    a2: i64,
    b2: i64,
    c2: i64,
    part2: bool,
) -> Option<(i64, i64)> {
    let determinant = a1 * b2 - a2 * b1;
    if determinant == 0 {
        return None;
    }

    // Check if solution exists by ensuring the determinant divides evenly
    if (c1 * b2 - c2 * b1) % determinant != 0 || (a1 * c2 - a2 * c1) % determinant != 0 {
        return None;
    }

    let x = (c1 * b2 - c2 * b1) / determinant;
    let y = (a1 * c2 - a2 * c1) / determinant;

    // For part 1, check the constraints
    if !part2 {
        if x >= 0 && y >= 0 && x <= 100 && y <= 100 {
            return Some((x, y));
        }
    } else {
        // For part 2, just check if positive
        if x >= 0 && y >= 0 {
            return Some((x, y));
        }
    }
    None
}

fn can_reach_prize(machine: &ClawMachine, part2: bool) -> Option<(i64, i64)> {
    // We need to solve:
    // a*button_a.x + b*button_b.x = prize.x
    // a*button_a.y + b*button_b.y = prize.y
    solve_linear_equations(
        machine.button_a.0,
        machine.button_b.0,
        machine.prize.0,
        machine.button_a.1,
        machine.button_b.1,
        machine.prize.1,
        part2,
    )
}
fn calculate_tokens(a_presses: i64, b_presses: i64) -> i64 {
    a_presses * 3 + b_presses
}

fn part1(input: &str) -> i64 {
    let machines = parse_input(input);
    let mut total_tokens = 0;

    for machine in machines {
        if let Some((a_presses, b_presses)) = can_reach_prize(&machine, false) {
            total_tokens += calculate_tokens(a_presses, b_presses);
        }
    }

    total_tokens
}

fn part2(input: &str) -> i64 {
    let machines = parse_input_2(input);
    let mut total_tokens = 0;

    for machine in machines {
        if let Some((a_presses, b_presses)) = can_reach_prize(&machine, true) {
            total_tokens += calculate_tokens(a_presses, b_presses);
        }
    }

    total_tokens
}

#[test]
fn test_part1() {
    let test_input = &include_str!("../data/test/13.txt").replace("\r", "");
    assert_eq!(part1(test_input), 480);
}
#[test]
fn test_part2() {
    let test_input = &include_str!("../data/test/13.txt").replace("\r", "");
    assert_eq!(part2(test_input), 875318608908);
}
