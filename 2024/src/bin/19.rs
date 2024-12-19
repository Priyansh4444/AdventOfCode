use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let input: &str = include_str!("../data/19.txt");
    println!("Answer to part1: {}", part1(input));
    println!("Answer to part2: {}", part2(input));
}

fn solve_design(
    design: &str,
    patterns: &HashSet<&str>,
    cache: &mut HashMap<String, Option<usize>>,
) -> Option<usize> {
    if design.is_empty() {
        return Some(1);
    }

    if let Some(&result) = cache.get(design) {
        return result;
    }

    let mut total = 0;
    let mut found = false;

    for pattern in patterns {
        if design.starts_with(pattern) {
            if let Some(count) = solve_design(&design[pattern.len()..], patterns, cache) {
                total += count;
                found = true;
            }
        }
    }

    let result = if found { Some(total) } else { None };
    cache.insert(design.to_string(), result);
    result
}

fn part1(input: &str) -> usize {
    let mut lines = input.lines();
    let patterns: HashSet<&str> = lines.next().unwrap().split(", ").collect();
    lines.next();

    let mut cache = HashMap::new();
    lines
        .filter(|design| solve_design(design, &patterns, &mut cache).is_some())
        .count()
}

fn part2(input: &str) -> usize {
    let mut lines = input.lines();
    let patterns: HashSet<&str> = lines.next().unwrap().split(", ").collect();
    lines.next();

    let mut cache = HashMap::new();
    lines
        .filter_map(|design| solve_design(design, &patterns, &mut cache))
        .sum()
}

#[test]
fn test_part1() {
    assert_eq!(part1(include_str!("../data/test/19.txt")), 6);
}

#[test]
fn test_part2() {
    assert_eq!(part2(include_str!("../data/test/19.txt")), 16);
}
