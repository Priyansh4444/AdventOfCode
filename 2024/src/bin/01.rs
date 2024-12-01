use std::collections::HashMap;

fn main() {
    let input: &str = include_str!("../data/01.txt");
    println!("Answer to part1: {}", part1(input));
    println!("Answer to part2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let mut left_list: Vec<usize> = Vec::new();
    let mut right_list: Vec<usize> = Vec::new();

    for line in input.lines() {
        let mut parts = line.split_whitespace();
        left_list.push(parts.next().unwrap().parse().unwrap());
        right_list.push(parts.next().unwrap().parse().unwrap());
    }

    left_list.sort_unstable();
    right_list.sort_unstable();

    left_list
        .iter()
        .zip(right_list.iter())
        .map(|(left, right)| (*left as isize - *right as isize).abs() as usize)
        .sum()
}

#[test]
fn test_part1() {
    assert_eq!(part1(include_str!("../data/01_test.txt")), 11);
}

fn part2(input: &str) -> usize {
    let mut left_list: Vec<usize> = Vec::new();
    let mut right_list: Vec<usize> = Vec::new();

    for line in input.lines() {
        let mut parts = line.split_whitespace();
        left_list.push(parts.next().unwrap().parse().unwrap());
        right_list.push(parts.next().unwrap().parse().unwrap());
    }

    let mut similarity_score = 0;
    let right_list = right_list.iter().fold(HashMap::new(), |mut acc, &x| {
        *acc.entry(x).or_insert(0) += 1;
        acc
    });
    for left in &left_list {
        let count = right_list.get(left).unwrap_or(&0);
        similarity_score += left * count;
    }

    similarity_score
}

#[cfg(test)]
#[test]
fn test_part2() {
    assert_eq!(part2(include_str!("../data/01_test.txt")), 31);
}
