fn main() {
    let input: &str = include_str!("../data/04.txt");
    println!("Answer to part1: {}", part1(input));
    println!("Answer to part2: {}", part2(input));
}

fn check_direction(
    grid: &Vec<Vec<char>>,
    row: usize,
    col: usize,
    row_dir: i32,
    col_dir: i32,
    word: &[&str],
) -> bool {
    let height = grid.len() as i32;
    let width = grid[0].len() as i32;

    for (i, &letter) in word.iter().enumerate() {
        let new_row = row as i32 + (row_dir * i as i32);
        let new_col = col as i32 + (col_dir * i as i32);

        if new_row < 0 || new_row >= height || new_col < 0 || new_col >= width {
            return false;
        }

        if grid[new_row as usize][new_col as usize] != letter.chars().next().unwrap() {
            return false;
        }
    }
    true
}

fn part1(input: &str) -> usize {
    let word = ["X", "M", "A", "S"];
    let mut count = 0;
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let height = grid.len();
    let width = grid[0].len();

    for row in 0..height {
        for col in 0..width {
            // Check all 8 directions
            let directions = [
                (0, 1),
                (0, -1),
                (1, 0),
                (-1, 0),
                (1, 1),
                (1, -1),
                (-1, 1),
                (-1, -1),
            ];
            // Loop through all 8 directions moving through them seeing if the word exists or its reverse exist
            for &(row_dir, col_dir) in directions.iter() {
                if check_direction(&grid, row, col, row_dir, col_dir, &word) {
                    count += 1;
                }
            }
        }
    }

    count
}

#[test]
fn test_part1() {
    assert_eq!(part1(include_str!("../data/test/04.txt")), 18);
}

fn part2(input: &str) -> usize {
    let word = ["M", "A", "S"];
    let mut count = 0;
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut grid_bools = vec![vec![0; grid[0].len()]; grid.len()];
    let height = grid.len();
    let width = grid[0].len();

    for row in 0..height {
        for col in 0..width {
            // Check all 4 diagonal directions
            let directions = [(1, 1), (1, -1), (-1, 1), (-1, -1)];
            // Loop through all 48 directions moving through them seeing if the word MAS exists or its reverse exist
            for &(row_dir, col_dir) in directions.iter() {
                check_direction_2(&grid, row, col, row_dir, col_dir, &word, &mut grid_bools);
            }
        }
    }

    for row in 0..height {
        for col in 0..width {
            if grid_bools[row][col] == 2 && grid[row][col] == 'A' {
                count += 1;
            }
        }
    }

    count
}

fn check_direction_2(
    grid: &Vec<Vec<char>>,
    row: usize,
    col: usize,
    row_dir: i32,
    col_dir: i32,
    word: &[&str],
    grid_bools: &mut Vec<Vec<i32>>,
) {
    let height = grid.len() as i32;
    let width = grid[0].len() as i32;

    for (i, &letter) in word.iter().enumerate() {
        let new_row = row as i32 + (row_dir * i as i32);
        let new_col = col as i32 + (col_dir * i as i32);

        if new_row < 0 || new_row >= height || new_col < 0 || new_col >= width {
            return;
        }

        if grid[new_row as usize][new_col as usize] != letter.chars().next().unwrap() {
            return;
        }
    }
    for (i, &_) in word.iter().enumerate() {
        grid_bools[(row as i32 + (row_dir * i as i32)) as usize]
            [(col as i32 + (col_dir * i as i32)) as usize] += 1;
    }
}


#[test]
fn test_part2() {
    assert_eq!(part2(include_str!("../data/test/04.txt")), 9);
}
