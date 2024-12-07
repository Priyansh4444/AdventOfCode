fn main() {
    let input: &str = &include_str!("../data/07.txt").replace("\r", "");
    println!("Answer to part1: {}", part1(input));
    println!("Answer to part2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let mut sum = 0;
    for line in input.lines() {
        let parts: Vec<&str> = line.split(": ").collect();
        let rhs = parts[0].parse::<usize>().unwrap();
        let lhs: Vec<usize> = parts[1]
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        if can_make_value(rhs, &lhs, false) {
            sum += rhs;
        }
    }
    sum
}

fn can_make_value(target: usize, numbers: &[usize], part2: bool) -> bool {
    if numbers.len() == 1 {
        return numbers[0] == target;
    }
    let mut ops = vec!["+", "*"];
    if part2 {
        ops.push("||");
    }
    let mut expressions = vec![(numbers[0], vec![])];

    for &num in &numbers[1..] {
        let mut new_expressions = vec![];
        for (val, ops_used) in expressions {
            for &op in &ops {
                let new_val = match op {
                    "+" => val + num,
                    "*" => val * num,
                    "||" => (val.to_string() + &num.to_string()).parse().unwrap(),
                    _ => unreachable!(),
                };
                let mut new_ops: Vec<&str> = ops_used.clone();
                new_ops.push(op);
                new_expressions.push((new_val, new_ops));
            }
        }
        expressions = new_expressions;
    }
    expressions.iter().any(|(val, _)| *val == target)
}

fn part2(input: &str) -> usize {
    let mut sum = 0;
    for line in input.lines() {
        let parts: Vec<&str> = line.split(": ").collect();
        let rhs = parts[0].parse::<usize>().unwrap();
        let lhs: Vec<usize> = parts[1]
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        if can_make_value(rhs, &lhs, true) {
            sum += rhs;
        }
    }
    sum
}

#[test]
fn test_part1() {
    assert_eq!(part1(include_str!("../data/test/07.txt")), 3749);
}

#[test]
fn test_part2() {
    assert_eq!(part2(include_str!("../data/test/07.txt")), 11387);
}
