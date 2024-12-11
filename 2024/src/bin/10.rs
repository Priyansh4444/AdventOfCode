use std::collections::HashSet;

fn main() {
    let input: &str = include_str!("../data/10.txt");
    let now = std::time::Instant::now();
    println!("Answer to part1: {}", part1(input));
    let elapsed = now.elapsed();
    println!("Time: {:?}", elapsed);
    let now = std::time::Instant::now();
    println!("Answer to part2: {}", part2(input));
    let elapsed = now.elapsed();
    println!("Time: {:?}", elapsed);
}

fn part1(input: &str) -> usize {
    // Parse grid
    let grid: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let rows = grid.len();
    let cols = grid[0].len();

    // Find trailheads
    let mut total_score = 0;
    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] == 0 {
                total_score += count_reachable_nines(&grid, r, c);
            }
        }
    }

    total_score
}

fn count_reachable_nines(grid: &Vec<Vec<u32>>, start_r: usize, start_c: usize) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut visited = HashSet::new();
    let mut reachable_nines = std::collections::HashSet::new();
    let mut queue = std::collections::VecDeque::new();

    // (row, col, current_height)
    queue.push_back((start_r, start_c, 0));
    visited.insert((start_r, start_c));

    while let Some((r, c, height)) = queue.pop_front() {
        if grid[r][c] == 9 {
            reachable_nines.insert((r, c));
            continue;
        }

        // Check neighbors
        for (dr, dc) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let new_r = r as i32 + dr;
            let new_c: i32 = c as i32 + dc;

            if new_r >= 0 && new_r < rows as i32 && new_c >= 0 && new_c < cols as i32 {
                let new_r = new_r as usize;
                let new_c = new_c as usize;

                if !visited.contains(&(new_r, new_c)) && grid[new_r][new_c] == height + 1 {
                    visited.insert((new_r, new_c));
                    queue.push_back((new_r, new_c, height + 1));
                }
            }
        }
    }

    reachable_nines.len()
}

fn dfs(
    grid: &Vec<Vec<u32>>,
    r: usize,
    c: usize,
    height: u32,
    path: &mut Vec<(usize, usize)>,
    paths: &mut HashSet<Vec<(usize, usize)>>,
) {
    if grid[r][c] == 9 {
        paths.insert(path.clone());
        return;
    }

    for (dr, dc) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
        let new_r = r as i32 + dr;
        let new_c = c as i32 + dc;

        if new_r >= 0 && new_r < grid.len() as i32 && new_c >= 0 && new_c < grid[0].len() as i32 {
            let new_r = new_r as usize;
            let new_c = new_c as usize;

            if !path.contains(&(new_r, new_c)) && grid[new_r][new_c] == height + 1 {
                path.push((new_r, new_c));
                dfs(grid, new_r, new_c, height + 1, path, paths);
                path.pop();
            }
        }
    }
}

fn count_unique_paths(grid: &Vec<Vec<u32>>, start_r: usize, start_c: usize) -> usize {
    let mut paths = HashSet::new();
    let mut path = vec![(start_r, start_c)];
    dfs(grid, start_r, start_c, 0, &mut path, &mut paths);
    paths.len()
}

fn part2(input: &str) -> usize {
    let grid: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let mut total_rating = 0;
    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            if grid[r][c] == 0 {
                total_rating += count_unique_paths(&grid, r, c);
            }
        }
    }

    total_rating
}

#[test]
fn test_part1() {
    assert_eq!(part1(include_str!("../data/test/10.txt")), 36);
}

#[test]
fn test_part2() {
    assert_eq!(part2(include_str!("../data/test/10.txt")), 81);
}
