use std::collections::{HashMap, HashSet};

fn main() {
    let input: &str = include_str!("../data/05.txt");
    let now = std::time::Instant::now();
    println!("Answer to part1: {}", part1(input));
    println!("Time: {:?}", now.elapsed());
    println!("Answer to part2: {}", part2(input));
    println!("Time: {:?}", now.elapsed());
}

fn check_sequence_valid(sequence: &[i32], rules: &HashMap<i32, HashSet<i32>>) -> bool {
    // for all the left elements in the sequence "left | right"
    for (i, &left) in sequence.iter().enumerate() {
        if let Some(must_follow) = rules.get(&left) {
            // get everything in the rule book which comes before left (aka the rights which are in a set)
            for &right in must_follow {
                if sequence.contains(&right) {
                    // Find position of num2 in the sequence, and if its after the current one, return false!!!
                    let pos2 = sequence.iter().position(|&x| x == right).unwrap();
                    if pos2 < i {
                        return false;
                    }
                }
            }
        }
    }
    true
}

fn part1(input: &str) -> i32 {
    let (rules_str, sequences_str) = input.split_once("\n\r\n").unwrap();
    let rules: HashMap<i32, HashSet<i32>> = rules_str
        .lines()
        .map(|line| line.split_once('|').unwrap())
        .fold(HashMap::new(), |mut acc, (left, right)| {
            acc.entry(left.trim().parse().unwrap())
                .or_insert_with(HashSet::new)
                .insert(right.trim().parse().unwrap());
            acc
        });
    let mut sum = 0;

    for line in sequences_str.lines() {
        let sequence: Vec<i32> = line.split(',').map(|x| x.trim().parse().unwrap()).collect();
        if check_sequence_valid(&sequence, &rules) {
            let mid = sequence[sequence.len() / 2];
            sum += mid;
        }
    }

    sum
}

fn part2(input: &str) -> i32 {
    let (rules_str, sequences_str) = input.split_once("\n\r\n").unwrap();
    let rules: HashMap<i32, HashSet<i32>> = rules_str
        .lines()
        .map(|line| line.split_once('|').unwrap())
        .fold(HashMap::new(), |mut acc, (left, right)| {
            acc.entry(left.trim().parse().unwrap())
                .or_insert_with(HashSet::new)
                .insert(right.trim().parse().unwrap());
            acc
        });
    let mut sum = 0;
    for line in sequences_str.lines() {
        let mut sequence: Vec<i32> = line.split(',').map(|x| x.trim().parse().unwrap()).collect();
        if !check_sequence_valid(&sequence, &rules) {
            // Same as part 1 till here

            // sort the sequence, where if the element is not in the set of the element before it, it comes before it
            sequence.sort_by(|a, b| {
                if rules.get(a).map_or(false, |set| set.contains(b)) {
                    std::cmp::Ordering::Greater
                } else {
                    std::cmp::Ordering::Less
                }
            });
            let mid = sequence[sequence.len() / 2];
            sum += mid;
        }
    }

    sum
}

#[cfg(test)]
#[test]
fn test_part1() {
    assert_eq!(part1(include_str!("../data/test/05.txt")), 143);
}

#[test]
fn test_part2() {
    assert_eq!(part2(include_str!("../data/test/05.txt")), 123);
}
