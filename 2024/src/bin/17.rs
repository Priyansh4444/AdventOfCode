fn main() {
    let input: &str = include_str!("../data/17.txt");
    println!("Answer to part1: {}", part1(input));
    println!("Answer to part2: {}", part2(input));
}

fn parse_input(input: &str) -> (u64, u64, u64, Vec<u64>) {
    let lines: Vec<&str> = input.lines().collect();
    let a = lines[0].split(": ").nth(1).unwrap().parse().unwrap();
    let b = lines[1].split(": ").nth(1).unwrap().parse().unwrap();
    let c = lines[2].split(": ").nth(1).unwrap().parse().unwrap();
    let program = lines[4][8..]
        .split(",")
        .map(|s| s.trim().parse().unwrap())
        .collect();
    (a, b, c, program)
}

fn combo_operand(operand: u64, a: u64, b: u64, c: u64) -> u64 {
    // mainly used for combo operands
    match operand {
        0..=3 => operand,
        4 => a,
        5 => b,
        6 => c,
        _ => panic!("Invalid operand"),
    }
}

fn part1(input: &str) -> String {
    let (mut a, mut b, mut c, program) = parse_input(input);
    let mut output = Vec::new();
    let mut ip = 0;
    while ip < program.len() {
        let opcode = program[ip];
        let operand = program[ip + 1];
        match opcode {
            0 => {
                // adv
                let power = combo_operand(operand, a, b, c);
                a /= 1 << power;
                ip += 2;
            }
            1 => {
                // bxl
                b ^= operand;
                ip += 2;
            }
            2 => {
                // bst
                b = combo_operand(operand, a, b, c) % 8;
                ip += 2;
            }
            3 => {
                // jnz
                if a != 0 {
                    ip = operand as usize;
                } else {
                    ip += 2;
                }
            }
            4 => {
                // bxc
                b ^= c;
                ip += 2;
            }
            5 => {
                // out
                let value = combo_operand(operand, a, b, c) % 8;
                output.push(value.to_string());
                ip += 2;
            }
            6 => {
                // bdv
                let power = combo_operand(operand, a, b, c);
                b = a / (1 << power);
                ip += 2;
            }
            7 => {
                // cdv
                let power = combo_operand(operand, a, b, c);
                c = a / (1 << power);
                ip += 2;
            }
            _ => panic!("Unknown opcode"),
        }
    }

    output.join(",")
}

fn find(target: &[u64], ans: u64, program: &[u64]) -> Option<u64> {
    if target.is_empty() {
        return Some(ans);
    }

    for trial in 0..8 {
        let a = ans << 3 | trial;
        let mut b = 0;
        let mut c = 0;
        let mut output = None;
        let mut adv3 = false;

        // Process instructions up to but not including final JNZ
        for ip in (0..program.len() - 2).step_by(2) {
            let opcode = program[ip];
            let operand = program[ip + 1];
            if cool {
                println!("{}: {} {} {} {} {}", ip, opcode, operand, a, b, c);
            }
            match opcode {
                0 => {
                    assert!(!adv3, "program has multiple ADVs");
                    assert_eq!(operand, 3, "program has ADV with operand other than 3");

                    adv3 = true;
                }
                1 => b ^= operand,
                2 => b = combo_operand(operand, a, b, c) % 8,
                3 => panic!("program has JNZ inside expected loop body"), // No JNZ allowed in loop body
                4 => b ^= c,
                5 => {
                    assert!(output.is_none(), "program has multiple OUT");
                    output = Some(combo_operand(operand, a, b, c) % 8);
                }
                6 => b = a >> combo_operand(operand, a, b, c),
                7 => c = a >> combo_operand(operand, a, b, c),
                _ => panic!("Invalid opcode"),
            }
        }
        // Check if output matches target and recurse
        if let Some(out) = output {
            if out == target[target.len() - 1] {
                if let Some(sub) = find(&target[..target.len() - 1], a, &program) {
                    return Some(sub);
                } else {
                    continue;
                }
            }
        }
    }
    None
}

fn part2(input: &str) -> usize {
    let (_, _, _, program) = parse_input(input);
    assert_eq!(
        &program[program.len() - 2..],
        &[3, 0],
        "Program must end with JNZ 0"
    );

    find(&program, 0, &program).unwrap() as usize
}

#[test]
fn test_part1() {
    assert_eq!(
        part1(include_str!("../data/test/17.txt")),
        "4,6,3,5,6,3,5,2,1,0"
    );
}

#[test]
fn test_part2() {
    assert_eq!(part2(include_str!("../data/test/17.txt")), 0);
}
