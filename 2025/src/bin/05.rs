use std::collections::BinaryHeap;
fn main() {
    let input: &str = include_str!("../data/05.txt");
    println!("Answer to part1: {}", part1(input));
    println!("Answer to part2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let mut ranges: BinaryHeap<(usize, usize)> = BinaryHeap::new();
    let mut count = 0;
    for line in input.lines() {
        if line.contains('-') == true {
            let mut parts = line.split('-');
            let start: usize = parts.next().unwrap().parse().unwrap();
            let end: usize = parts.next().unwrap().parse().unwrap();
            ranges.push((end, start));
            continue;
        }
        if line.trim().is_empty() {
            continue;
        }
        let ingredient_id = line.parse::<usize>().unwrap();
        count += if ranges
            .iter()
            .any(|&(end, start)| ingredient_id >= start && ingredient_id <= end)
        {
            1
        } else {
            0
        };
    }

    count
}

fn part2(input: &str) -> usize {
    let mut ranges_vec: Vec<(usize, usize)> = Vec::new();
    for line in input.lines() {
        if line.contains('-') == true {
            let mut parts = line.split('-');
            let start: usize = parts.next().unwrap().parse().unwrap();
            let end: usize = parts.next().unwrap().parse().unwrap();
            ranges_vec.push((start, end));
            continue;
        }
        break;
    }

    ranges_vec.sort_by(|a, b| {
        let ord = a.0.cmp(&b.0);
        if ord == std::cmp::Ordering::Equal {
            a.1.cmp(&b.1)
        } else {
            ord
        }
    });

    let mut merged_ranges: Vec<(usize, usize)> = Vec::new();
    for (start, end) in ranges_vec {
        if let Some((_, last_end)) = merged_ranges.last_mut() {
            if start <= *last_end + 1 {
                *last_end = (*last_end).max(end);
            } else {
                merged_ranges.push((start, end));
            }
        } else {
            merged_ranges.push((start, end));
        }
    }

    let mut count = 0;
    for (start, end) in merged_ranges {
        if end >= start {
            count += end - start + 1;
        }
    }

    count
}

#[test]
fn test_part1() {
    assert_eq!(part1(include_str!("../data/test/05.txt")), 3);
}

#[test]
fn test_part2() {
    assert_eq!(part2(include_str!("../data/test/05.txt")), 014);
}
