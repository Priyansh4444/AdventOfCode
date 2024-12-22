fn main() {
    let input: &str = include_str!("../data/22.txt");
    println!("Answer to part1: {}", part1(input));
    println!("Answer to part2: {}", part2(input));
}

use std::collections::{HashMap, HashSet};

fn generate_next(n: u64) -> u64 {
    let mut n = n;
    n ^= n * 64;
    n %= 16777216;
    n ^= n / 32;
    n %= 16777216;
    n ^= n * 2048;
    n %= 16777216;
    n
}

fn generate_sequence(start: u64, count: u64) -> u64 {
    let mut n = start;
    for _ in 0..count {
        n = generate_next(n);
    }
    n
}

fn part1(input: &str) -> u64 {
    input
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .map(|n| generate_sequence(n, 2000))
        .sum()
}

fn part2(input: &str) -> u64 {
    let mut seq_to_total = HashMap::new();
    // Process each input number
    for num in input.lines().map(|l| l.parse::<u64>().unwrap()) {
        // Generate sequence
        let mut buyer = vec![num % 10];
        let mut n = num;
        for _ in 0..2000 {
            n = generate_next(n);
            buyer.push(n % 10);
        }
        // Find patterns
        let mut seen = HashSet::new();
        for window in buyer.windows(5) {
            let seq = [
                window[1] as i64 - window[0] as i64,
                window[2] as i64 - window[1] as i64,
                window[3] as i64 - window[2] as i64,
                window[4] as i64 - window[3] as i64,
            ];
            if seen.contains(&seq) {
                continue;
            }
            seen.insert(seq.clone());
            *seq_to_total.entry(seq).or_insert(0) += window[4] as u64;
        }
    }
    // Find maximum total
    seq_to_total.values().max().copied().unwrap()
}

#[test]
fn test_part1() {
    assert_eq!(part1(include_str!("../data/test/22.txt")), 37327623);
}

#[test]
fn test_part2() {
    assert_eq!(part2(include_str!("../data/test/22.txt")), 0);
}
