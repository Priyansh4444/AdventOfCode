fn main() {
    let input: &str = include_str!("../data/20.txt");
    println!("Answer to part1: {}", part1(input));
    println!("Answer to part2: {}", part2(input));
}

pub fn find_position(grid: &[Vec<char>], target: char) -> Option<(usize, usize)> {
    for (row_idx, row) in grid.iter().enumerate() {
        for (col_idx, &cell) in row.iter().enumerate() {
            if cell == target {
                return Some((row_idx, col_idx));
            }
        }
    }
    None
}

pub fn get_neighbors(row: usize, col: usize, rows: usize, cols: usize) -> Vec<(usize, usize, i32)> {
    let mut neighbors = Vec::new();
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    for (dx, dy) in directions {
        let new_row = row as i32 + dx;
        let new_col = col as i32 + dy;

        if new_row >= 0 && new_row < rows as i32 && new_col >= 0 && new_col < cols as i32 {
            neighbors.push((new_row as usize, new_col as usize, 1));
        }
    }

    neighbors
}

pub fn calculate_distances(
    grid: &[Vec<char>],
    start: (usize, usize),
    end: (usize, usize),
    rows: usize,
    cols: usize,
) -> Vec<Vec<i32>> {
    let mut distances = vec![vec![0; cols]; rows];
    let mut current_pos = start;
    let mut previous_pos = current_pos;

    while current_pos != end {
        for (next_row, next_col, _) in get_neighbors(current_pos.0, current_pos.1, rows, cols) {
            if grid[next_row][next_col] == '#' || (next_row, next_col) == previous_pos {
                continue;
            }
            distances[next_row][next_col] = distances[current_pos.0][current_pos.1] + 1;
            previous_pos = current_pos;
            current_pos = (next_row, next_col);
            break;
        }
    }

    distances
}

pub fn count_valid_cheats(
    grid: &[Vec<char>],
    distances: &[Vec<i32>],
    rows: usize,
    cols: usize,
    max_cheat_distance: usize,
    min_time_saved: i32,
) -> usize {
    let mut valid_cheats = 0;

    for start_row in 0..rows {
        for start_col in 0..cols {
            if grid[start_row][start_col] == '#' {
                continue;
            }

            for end_row in 0..rows {
                for end_col in 0..cols {
                    // Skip if it is a segmentation faults
                    if grid[end_row][end_col] == '#' {
                        continue;
                    }
                    // if you can skip the distance find the amount of time skipped by the path
                    let distance_skipped =
                        start_row.abs_diff(end_row) + start_col.abs_diff(end_col);
                    let time_saved = distances[end_row][end_col]
                        - distances[start_row][start_col]
                        - distance_skipped as i32;

                    if distance_skipped <= max_cheat_distance && time_saved >= min_time_saved {
                        valid_cheats += 1;
                    }
                }
            }
        }
    }

    valid_cheats
}

pub fn part1(input: &str) -> usize {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let rows = grid.len();
    let cols = grid[0].len();
    let start_pos = find_position(&grid, 'S').unwrap();
    let end_pos = find_position(&grid, 'E').unwrap();

    // Calculate distances from start to end
    let distances = calculate_distances(&grid, start_pos, end_pos, rows, cols);
    let cheat_count = count_valid_cheats(&grid, &distances, rows, cols, 2, 100);

    cheat_count
}

pub fn part2(input: &str) -> usize {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let rows = grid.len();
    let cols = grid[0].len();
    let start_pos = find_position(&grid, 'S').unwrap();
    let end_pos = find_position(&grid, 'E').unwrap();

    let distances = calculate_distances(&grid, start_pos, end_pos, rows, cols);
    let cheat_count = count_valid_cheats(&grid, &distances, rows, cols, 20, 100);

    cheat_count
}

#[test]
fn test_part1() {
    assert_eq!(part1(include_str!("../data/test/20.txt")), 0);
}

#[test]
fn test_part2() {
    assert_eq!(part2(include_str!("../data/test/20.txt")), 0);
}
