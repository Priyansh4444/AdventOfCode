fn main() {
    let input: &str = &include_str!("../data/07.txt").replace("\r", "");
    println!("Answer to part1: {}", part1(input));
    println!("Answer to part2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let now = std::time::Instant::now();
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
    println!("Time: {:?}", now.elapsed());
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
    let mut expressions = vec![numbers[0]];

    for &num in &numbers[1..] {
        let mut new_expressions = vec![];
        for val in expressions {
            for &op in &ops {
                let new_val = match op {
                    "+" => val + num,
                    "*" => val * num,
                    "||" => (val.to_string() + &num.to_string()).parse().unwrap(),
                    _ => unreachable!(),
                };
                if new_val <= target {
                    new_expressions.push(new_val);
                }
            }
        }
        expressions = new_expressions;
    }
    expressions.iter().any(|val| *val == target)
}

fn part2(input: &str) -> usize {
    let now: std::time::Instant = std::time::Instant::now();
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
    println!("Time: {:?}", now.elapsed());
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
