use std::collections::{HashSet, VecDeque};

fn main() {
    // Load input data, removing any carriage return characters
    let input: &str = &include_str!("../data/15.txt").replace("\r", "");
    println!("Answer to part1: {}", part1(input));
    println!("Answer to part2: {}", part2(input));
}

// Part 1 of the problem
fn part1(input: &str) -> usize {
    let (grid, instructions) = parse_input(input);
    solve(grid, instructions)
}

// Part 2 of the problem
fn part2(input: &str) -> usize {
    let (grid, instructions) = parse_input(input);

    // Transform the grid for part 2
    let transformed_grid = grid
        .iter()
        .map(|line| {
            line.iter()
                .flat_map(|&cell| match cell {
                    b'#' => b"##",
                    b'O' => b"[]",
                    b'.' => b"..",
                    b'@' => b"@.",
                    _ => unreachable!(),
                })
                .copied()
                .collect()
        })
        .collect();

    solve(transformed_grid, instructions)
}

fn parse_input(input: &str) -> (Vec<Vec<u8>>, &str) {
    let (grid_data, instructions) = input.split_once("\n\n").unwrap();
    let grid = grid_data
        .lines()
        .map(|line| line.as_bytes().to_vec())
        .collect();
    (grid, instructions)
}

fn solve(mut grid: Vec<Vec<u8>>, instructions: &str) -> usize {
    //  Robot found
    let mut row = 0;
    let mut col = 0;
    for r in 0..grid.len() {
        for c in 0..grid[r].len() {
            if grid[r][c] == b'@' {
                row = r;
                col = c;
                grid[r][c] = b'.';
                break;
            }
        }
    }
    for instruction in instructions.bytes() {
        let (delta_row, delta_col) = match instruction {
            b'^' => (-1, 0),
            b'>' => (0, 1),
            b'v' => (1, 0),
            b'<' => (0, -1),
            _ => continue,
        };

        // queue for everything that needs to be moved in the direction
        let mut queue = VecDeque::from([(row, col)]);
        let mut visited = HashSet::new();
        let mut hit_wall = false;

        while let Some((current_row, current_col)) = queue.pop_front() {
            // If this position has been visited, skip it
            if !visited.insert((current_row, current_col)) {
                continue;
            }
            let (next_row, next_col) = (
                (current_row as isize + delta_row) as usize,
                (current_col as isize + delta_col) as usize,
            );
            match grid.get(next_row).and_then(|row| row.get(next_col)) {
                Some(&b'#') => {
                    // if wall hit like stop nothing in life matters
                    hit_wall = true;
                    break;
                }
                // You need to be able to push both the O and the [] each
                Some(&b'O') => queue.push_back((next_row, next_col)),
                Some(&b'[') => queue.extend([(next_row, next_col), (next_row, next_col + 1)]),
                Some(&b']') => queue.extend([(next_row, next_col), (next_row, next_col - 1)]),
                _ => continue,
            }
        }

        if hit_wall {
            continue;
        }

        // Move the visited positions to the new locations on the grid
        while !visited.is_empty() {
            for &(current_row, current_col) in &visited.clone() {
                let (next_row, next_col) = (
                    (current_row as isize + delta_row) as usize,
                    (current_col as isize + delta_col) as usize,
                );
                if !visited.contains(&(next_row, next_col)) {
                    grid[next_row][next_col] = grid[current_row][current_col];
                    grid[current_row][current_col] = b'.';
                    visited.remove(&(current_row, current_col));
                }
            }
        }

        row = (delta_row + row as isize) as usize;
        col = (delta_col + col as isize) as usize;
    }

    let mut sum = 0;

    for r in 0..grid.len() {
        for c in 0..grid[r].len() {
            if grid[r][c] == b'O' || grid[r][c] == b'[' {
                sum += r * 100 + c;
            }
        }
    }

    sum
}

#[test]
fn test_part1() {
    // Test part1 with expected result
    assert_eq!(
        part1(&include_str!("../data/test/15.txt").replace("\r", "")),
        10092 // Replace with correct expected value
    );
}

#[test]
fn test_part2() {
    // Test part2 with expected result
    assert_eq!(
        part2(&include_str!("../data/test/15.txt").replace("\r", "")),
        9021 // Replace with correct expected value
    );
}
