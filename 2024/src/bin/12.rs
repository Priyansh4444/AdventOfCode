use core::time;
use std::collections::{HashSet, VecDeque};

fn main() {
    let input: &str = include_str!("../data/12.txt");
    let now = std::time::Instant::now();
    println!("Answer to part1: {}", part1(input));
    let elapsed = now.elapsed();
    println!("Elapsed: {:?}", elapsed);
    let now = std::time::Instant::now();
    println!("Answer to part2: {}", part2(input));
    let elapsed = now.elapsed();
    println!("Elapsed: {:?}", elapsed);
}

fn search_area_perim(
    grid: &Vec<Vec<char>>,
    start_r: usize,
    start_c: usize,
    plant_type: char,
    visited: &mut HashSet<(usize, usize)>,
) -> (usize, usize) {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut queue = VecDeque::new();
    let mut area = 0;
    let mut perimeter = 0;

    queue.push_back((start_r, start_c));
    visited.insert((start_r, start_c));

    while let Some((r, c)) = queue.pop_front() {
        area += 1;
        // assume you add no bounds
        let mut local_perimeter = 4;

        for (dr, dc) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let new_r = r as isize + dr;
            let new_c = c as isize + dc;

            if new_r >= 0 && new_r < rows as isize && new_c >= 0 && new_c < cols as isize {
                let new_r = new_r as usize;
                let new_c = new_c as usize;
                // if you add bounds then subtract your own perimeter by 1 but the next one can have them all
                if grid[new_r][new_c] == plant_type {
                    local_perimeter -= 1;
                    if !visited.contains(&(new_r, new_c)) {
                        visited.insert((new_r, new_c));
                        queue.push_back((new_r, new_c));
                    }
                }
            }
        }

        perimeter += local_perimeter;
    }

    (area, perimeter)
}

fn part1(input: &str) -> usize {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let rows = grid.len();
    let cols = grid[0].len();
    let mut visited = HashSet::new();
    let mut total_price = 0;

    for r in 0..rows {
        for c in 0..cols {
            if !visited.contains(&(r, c)) {
                let plant_type = grid[r][c];
                let (area, perimeter) = search_area_perim(&grid, r, c, plant_type, &mut visited);
                total_price += area * perimeter;
            }
        }
    }

    total_price
}

fn search_area(
    grid: &Vec<Vec<char>>,
    row: usize,
    col: usize,
    plant: char,
    visited: &mut HashSet<(usize, usize)>,
) -> usize {
    let mut area = 1;
    let mut queue = vec![(row, col)];
    visited.insert((row, col));

    while let Some((r, c)) = queue.pop() {
        for (dr, dc) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let nr = r as i32 + dr;
            let nc = c as i32 + dc;

            if nr >= 0 && nr < grid.len() as i32 && nc >= 0 && nc < grid[0].len() as i32 {
                let pos = (nr as usize, nc as usize);
                if !visited.contains(&pos) && grid[pos.0][pos.1] == plant {
                    visited.insert(pos);
                    queue.push(pos);
                    area += 1;
                }
            }
        }
    }
    area
}

fn sides_count(visited: &HashSet<(usize, usize)>) -> usize {
    let mut seen = HashSet::new();
    for &(row, col) in visited {
        for (dr, dc) in [(-1, 0), (0, 1), (1, 0), (0, -1)] {
            if visited.contains(&(row + dr as usize, col + dc as usize)) {
                continue;
            }
            // first out of bounds element in this direction
            let (mut next_row, mut next_col) = (row, col);
            // Follow edge while:
            // - while the next element in this direction is not in bounds (since we are continuing out of bounds rows)
            // - while the perpendicular element is in bounds
            while visited.contains(&(next_row + dc as usize, next_col + dr as usize))
                && !visited.contains(&(next_row + dr as usize, next_col + dc as usize))
            {
                next_row += dc as usize;
                next_col += dr as usize;
            }
            seen.insert((next_row, next_col, dr, dc));
        }
    }
    seen.len()
}

fn part2(input: &str) -> usize {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut visited = HashSet::new();
    let mut total_price = 0;

    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            if !visited.contains(&(r, c)) && grid[r][c] != ' ' {
                let mut region = HashSet::new();
                let area = search_area(&grid, r, c, grid[r][c], &mut region);
                let sides = sides_count(&region);
                total_price += area * sides;
                visited.extend(region);
            }
        }
    }

    total_price
}

#[test]
fn test_part1() {
    assert_eq!(part1(include_str!("../data/test/12.txt")), 1930);
}

#[test]
fn test_part2() {
    assert_eq!(part2(include_str!("../data/test/12.txt")), 1206);
}
