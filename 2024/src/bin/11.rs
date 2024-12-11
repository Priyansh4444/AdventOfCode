use std::collections::HashMap;

fn main() {
    let input: &str = include_str!("../data/11.txt");
    let now = std::time::Instant::now();
    println!("Answer to part1: {}", part1(input));
    let elapsed = now.elapsed();
    println!("Time: {:?}", elapsed);
    let now = std::time::Instant::now();
    println!("Answer to part2: {}", part2(input));
    let elapsed = now.elapsed();
    println!("Time: {:?}", elapsed);
}

fn split_number(n: usize) -> (usize, usize) {
    let s = n.to_string();
    let half = s.len() / 2;
    let left = &s[..half];
    let right = &s[half..];
    (left.parse().unwrap(), right.parse().unwrap())
}

fn part1(input: &str) -> usize {
    let mut stones: Vec<usize> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    for _ in 0..25 {
        stones = stones
            .into_iter()
            .flat_map(|n| {
                if n == 0 {
                    vec![1]
                } else if n.to_string().len() % 2 == 0 {
                    let (left, right) = split_number(n);
                    vec![left, right]
                } else {
                    vec![n * 2024]
                }
            })
            .collect();
    }

    stones.len()
}

fn part2(input: &str) -> usize {
    // storing frequency of stones
    let mut stones: HashMap<usize, usize> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .fold(HashMap::new(), |mut map, stone| {
            *map.entry(stone).or_insert(0) += 1;
            map
        });

    let mut new_stones = HashMap::new();
    for _ in 0..75 {
        // clear new_stones
        new_stones.clear();
        // iterate over stones in the frequency map
        for (&stone, &count) in stones.iter() {
            // if stone is 0, increment frequency of 1 by count
            if stone == 0 {
                *new_stones.entry(1).or_insert(0) += count;
            } else if stone.to_string().len() % 2 == 0 {
                // split the stone into two halves and increment their frequencies by count
                let (left, right) = split_number(stone);
                *new_stones.entry(left).or_insert(0) += count;
                *new_stones.entry(right).or_insert(0) += count;
            } else {
                // multiply the stone by 2024 and increment its frequency by countS
                *new_stones.entry(stone * 2024).or_insert(0) += count;
            }
        }
        stones = new_stones.clone();
    }

    stones.values().sum()
}

#[test]
fn test_part1() {
    assert_eq!(part1(include_str!("../data/test/11.txt")), 55312);
}

#[test]
fn test_part2() {
    assert_eq!(part2(include_str!("../data/test/11.txt")), 65601038650482);
}
