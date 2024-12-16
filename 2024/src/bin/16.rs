use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet, VecDeque};

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: i32,
    position: (usize, usize),
    direction: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost) // Reverse for min-heap
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

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

fn get_next_moves(
    pos: (usize, usize),
    dir: usize,
    grid: &[Vec<char>],
) -> Vec<((usize, usize), usize)> {
    let directions: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut moves = Vec::new();

    // Can only turn 90 degrees or continue straight
    for new_dir in [(dir + 3) % 4, dir, (dir + 1) % 4].iter() {
        let (dr, dc) = directions[*new_dir];
        let new_r = pos.0 as i32 + dr;
        let new_c = pos.1 as i32 + dc;

        if new_r >= 0
            && new_r < grid.len() as i32
            && new_c >= 0
            && new_c < grid[0].len() as i32
            && grid[new_r as usize][new_c as usize] != '#'
        {
            moves.push(((new_r as usize, new_c as usize), *new_dir));
        }
    }
    moves
}

fn main() {
    let time = std::time::Instant::now();
    let input = include_str!("../data/16.txt");
    println!("Part 1: {}", part1(input));
    println!("Time: {}ms", time.elapsed().as_millis());
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> i32 {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let rows = grid.len();
    let columns = grid[0].len();

    let start = find_char(&grid, 'S').unwrap_or((rows - 2, 1));
    let end = find_char(&grid, 'E').unwrap_or((1, columns - 2));

    let mut costs = vec![vec![[i32::MAX; 4]; columns]; rows];
    let mut heap = BinaryHeap::new();

    // Start facing east (direction 0)
    costs[start.0][start.1][0] = 0;
    heap.push(State {
        cost: 0,
        position: start,
        direction: 0,
    });

    while let Some(State {
        cost,
        position,
        direction,
    }) = heap.pop()
    {
        if position == end {
            return cost;
        }

        if cost > costs[position.0][position.1][direction] {
            continue;
        }

        // Try all valid moves from current position
        for (new_pos, new_dir) in get_next_moves(position, direction, &grid) {
            let new_cost = cost + if new_dir == direction { 1 } else { 1001 };

            if new_cost < costs[new_pos.0][new_pos.1][new_dir] {
                heap.push(State {
                    cost: new_cost,
                    position: new_pos,
                    direction: new_dir,
                });
                costs[new_pos.0][new_pos.1][new_dir] = new_cost;
            }
        }
    }
    i32::MAX
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
