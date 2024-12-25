use std::collections::HashSet;

fn main() {
    let input: &str = &include_str!("../data/25.txt").replace("\r", "");
    println!("Answer to part1: {}", part1(input));
}

fn pins(grid: Vec<Vec<char>>) -> Vec<u8> {
    let mut out = Vec::new();
    for col in 0..grid[0].len() {
        out.push(grid.iter().filter(|row| row[col] == '#').count() as u8 - 1)
    }
    out
}

fn part1(input: &str) -> usize {
    let items: Vec<_> = input
        .split("\n\n")
        .map(|s| {
            s.lines()
                .map(|line| line.chars().collect::<Vec<_>>())
                .collect::<Vec<_>>()
        })
        .collect();

    let mut locks = Vec::new();
    let mut keys = Vec::new();

    for grid in items {
        if grid.first().unwrap().iter().all(|&x| x == '#')
            && grid.last().unwrap().iter().all(|&x| x == '.')
        {
            locks.push(pins(grid));
        } else {
            keys.push(pins(grid));
        }
    }

    let mut matching = 0;
    for key in &keys {
        for lock in &locks {
            if key.iter().zip(lock.iter()).all(|(a, b)| a + b <= 5) {
                matching += 1;
            }
        }
    }
    matching
}

#[test]
fn test_part1() {
    assert_eq!(
        part1(&include_str!("../data/test/25.txt").replace("\r", "")),
        3
    );
}
