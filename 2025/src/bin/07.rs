use std::collections::{HashMap, HashSet};

fn main() {
    let input: &str = include_str!("../data/07.txt");
    println!("Answer to part1: {}", part1(input));
    println!("Answer to part2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let start = grid[0].iter().position(|v| *v == 'S').unwrap();

    let mut beams: HashSet<usize> = HashSet::new();
    beams.insert(start);

    let mut total_splits = 0;

    // Simulate beam propagation row by row starting after S
    for r in 1..grid.len() {
        let mut next_beams = HashSet::new();
        let row_width = grid[r].len();

        for &c in &beams {
            if c >= row_width {
                continue;
            }

            match grid[r][c] {
                '^' => {
                    total_splits += 1;
                    if c > 0 {
                        next_beams.insert(c - 1);
                    }
                    if c + 1 < row_width {
                        next_beams.insert(c + 1);
                    }
                }
                _ => {
                    next_beams.insert(c);
                }
            }
        }
        beams = next_beams;
    }

    total_splits
}

fn part2(input: &str) -> usize {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let start = grid[0].iter().position(|v| *v == 'S').unwrap();
    // col -> count of active timelines ending at this column
    let mut timelines: HashMap<usize, usize> = HashMap::new();
    timelines.insert(start, 1);

    let row_width = grid[0].len();

    for r in 1..grid.len() {
        let mut next_timelines: HashMap<usize, usize> = HashMap::new();

        for (&c, &count) in &timelines {

            match grid[r][c] {
                '^' => {
                    if c > 0 {
                        *next_timelines.entry(c - 1).or_insert(0) += count;
                    }

                    if c + 1 < row_width {
                        *next_timelines.entry(c + 1).or_insert(0) += count;
                    }
                }
                _ => {
                    *next_timelines.entry(c).or_insert(0) += count;
                }
            }
        }
        timelines = next_timelines;
    }
    dbg!(timelines.values().sum::<usize>());
    timelines.values().sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(include_str!("../data/test/07.txt")), 21);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(include_str!("../data/test/07.txt")), 40);
    }
}
