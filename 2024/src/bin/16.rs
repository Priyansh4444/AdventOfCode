use std::collections::HashSet;
use std::collections::VecDeque;

struct Bot {
    location: (usize, usize),
    direction: usize,
    rows: usize,
    columns: usize,
    score: i32,
    visited: Vec<(usize, usize)>,
}

impl Bot {
    // Returns possible next locations and directions
    fn next_locations(&self) -> Vec<((usize, usize), usize)> {
        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        directions
            .iter()
            .enumerate()
            .map(|(i, &(dr, dc))| {
                let new_row = (self.location.0 as i32 + dr) as usize;
                let new_column = (self.location.1 as i32 + dc) as usize;
                ((new_row, new_column), i)
            })
            .filter(|&((new_row, new_column), _)| new_row < self.rows && new_column < self.columns)
            .collect()
    }

    // Visualizes the path taken by the bot
    fn visualize_path(&self, grid: &[Vec<char>]) -> Vec<Vec<char>> {
        let mut visual_grid = grid.to_vec();
        for &(row, col) in &self.visited {
            visual_grid[row][col] = '*';
        }
        visual_grid
    }

    fn print_path(&self, grid: &[Vec<char>]) {
        let visual_grid = self.visualize_path(grid);
        for row in visual_grid {
            for cell in row {
                print!("{}", cell);
            }
            println!();
        }
    }
}

fn main() {
    let input = include_str!("../data/16.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> i32 {
    // Parse input into grid
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let rows = grid.len();
    let columns = grid[0].len();

    let mut scores = vec![vec![i32::MAX; columns]; rows];

    // Find start and end positions
    let start_location = find_char(&grid, 'S').unwrap_or((rows - 2, 1));
    let end_location = find_char(&grid, 'E').unwrap_or((1, columns - 2));

    let mut queue = VecDeque::new();
    queue.push_back(Bot {
        location: start_location,
        direction: 0,
        rows,
        columns,
        score: 0,
        visited: vec![],
    });
    scores[start_location.0][start_location.1] = 0;

    // BFS
    while let Some(bot) = queue.pop_front() {
        for ((new_row, new_column), new_direction) in bot.next_locations() {
            if grid[new_row][new_column] == '#' {
                continue;
            }

            let mut new_score = bot.score + 1;
            if new_direction != bot.direction {
                new_score += 1000;
            }

            if new_score < scores[new_row][new_column] {
                scores[new_row][new_column] = new_score;
                queue.push_back(Bot {
                    location: (new_row, new_column),
                    direction: new_direction,
                    rows,
                    columns,
                    score: new_score,
                    visited: vec![],
                });
            }
        }
    }
    scores[end_location.0][end_location.1]
}

fn part2(input: &str) -> i32 {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let rows = grid.len();
    let columns = grid[0].len();

    let mut scores = vec![vec![[i32::MAX; 2]; columns]; rows];
    let start_location = find_char(&grid, 'S').unwrap_or((rows - 2, 1));
    let end_location = find_char(&grid, 'E').unwrap_or((1, columns - 2));

    let mut queue = VecDeque::new();
    queue.push_back(Bot {
        location: start_location,
        direction: 0,
        rows,
        columns,
        score: 0,
        visited: vec![(start_location.0, start_location.1)],
    });

    // getting the minimum score
    let end_score = part1(input);
    let mut end_visited_paths = vec![];

    // BFS
    while let Some(mut bot) = queue.pop_front() {
        for ((new_row, new_column), new_direction) in bot.next_locations() {
            if grid[new_row][new_column] == '#' {
                continue;
            }

            let mut new_score = bot.score + 1;
            if new_direction != bot.direction {
                new_score += 1000;
            }
            // if it is the same score as part 1 then add it to the visited paths
            if (new_row, new_column) == end_location && new_score == end_score {
                bot.visited.push((new_row, new_column));
                end_visited_paths.push(bot.visited.clone());
            }
            // else if it is the same score as the first one then change the best score for that region to this new one
            else if new_score <= scores[new_row][new_column][new_direction % 2] {
                scores[new_row][new_column][new_direction % 2] = new_score;
                let mut visited = bot.visited.clone();
                visited.push((new_row, new_column));
                queue.push_back(Bot {
                    location: (new_row, new_column),
                    direction: new_direction,
                    rows,
                    columns,
                    score: new_score,
                    visited,
                });
                // println!("Score: {}", new_score);
                // bot.print_path(&grid);
                // println!();
            }
        }
    }

    count_unique_visited(&end_visited_paths, rows, columns)
}

fn find_char(grid: &[Vec<char>], target: char) -> Option<(usize, usize)> {
    for (row_index, row) in grid.iter().enumerate() {
        for (column_index, &character) in row.iter().enumerate() {
            if character == target {
                return Some((row_index, column_index));
            }
        }
    }
    None
}

// Counts the number of unique visited cells in the paths
fn count_unique_visited(paths: &[Vec<(usize, usize)>], _rows: usize, _columns: usize) -> i32 {
    paths
        .iter()
        .flat_map(|path| path.iter())
        .collect::<HashSet<_>>()
        .len() as i32
}

#[test]
fn test_part1() {
    assert_eq!(part1(include_str!("../data/test/16.txt")), 11048);
}

#[test]
fn test_part2() {
    assert_eq!(part2(include_str!("../data/test/16.txt")), 64);
}
