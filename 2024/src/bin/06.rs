fn main() {
    let input: &str = include_str!("../data/06.txt");
    println!("Answer to part1: {}", part1(input));
    println!("Answer to part2: {}", part2(input));
}

fn find_starting_point(grid: &Vec<Vec<char>>) -> (usize, usize) {
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == '^' {
                return (x, y);
            }
        }
    }
    (0, 0)
}

fn part1(input: &str) -> usize {
    let laser_grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut visited = vec![vec![false; laser_grid[0].len()]; laser_grid.len()];
    let (mut x, mut y) = find_starting_point(&laser_grid);
    let rows = laser_grid.len();
    let cols = laser_grid[0].len();

    let dirs = [(0, -1), (1, 0), (0, 1), (-1, 0)];
    let mut moved = 1;
    let mut current_dir = 0;
    loop {
        let (dx, dy) = dirs[current_dir];
        let next_x = x as i32 + dx;
        let next_y = y as i32 + dy;

        // Check if next position would be outside bounds
        if next_x < 0 || next_x >= cols as i32 || next_y < 0 || next_y >= rows as i32 {
            break;
        }

        let next_pos = laser_grid[next_y as usize][next_x as usize];

        if next_pos == '#' {
            // Turn right when hitting obstacle
            current_dir = (current_dir + 1) % 4;
        } else {
            // Move forward
            x = next_x as usize;
            y = next_y as usize;
            if !visited[y][x] {
                visited[y][x] = true;
                moved += 1;
            }
        }
    }
    moved
}

fn part2(input: &str) -> usize {
    let now = std::time::Instant::now();
    let mut grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let (mut x, mut y) = find_starting_point(&grid);
    let rows = grid.len();
    let cols = grid[0].len();
    let mut loop_positions = std::collections::HashSet::new();

    let dirs = [(0, -1), (1, 0), (0, 1), (-1, 0)];
    let mut current_dir = 0;

    loop {
        let (dx, dy) = dirs[current_dir];
        let next_x = x as i32 + dx;
        let next_y = y as i32 + dy;

        // Check boundaries
        if next_x < 0 || next_x >= cols as i32 || next_y < 0 || next_y >= rows as i32 {
            break;
        }

        let next_x = next_x as usize;
        let next_y = next_y as usize;

        if grid[next_y][next_x] == '#' {
            current_dir = (current_dir + 1) % 4;
        } else {
            if check_makes_loop(&mut grid, next_x, next_y) {
                loop_positions.insert((next_x, next_y));
            }
            x = next_x;
            y = next_y;
        }
    }
    let elapsed = now.elapsed();
    println!("Elapsed: {:?}", elapsed);
    loop_positions.len()
}

fn check_makes_loop(grid: &mut Vec<Vec<char>>, x: usize, y: usize) -> bool {
    let original = grid[y][x];
    if original != '.' {
        return false;
    }
    grid[y][x] = '#';
    let mut visited = std::collections::HashSet::new();
    let (mut curr_x, mut curr_y) = find_starting_point(&grid);
    let mut dir = 0;
    let dirs = [(0, -1), (1, 0), (0, 1), (-1, 0)];

    loop {
        if !visited.insert((curr_x, curr_y, dir)) {
            grid[y][x] = original;
            return true;
        }

        let (dx, dy) = dirs[dir];
        let next_x = curr_x as i32 + dx;
        let next_y = curr_y as i32 + dy;

        if next_x < 0 || next_x >= grid[0].len() as i32 || next_y < 0 || next_y >= grid.len() as i32
        {
            grid[y][x] = original;
            return false;
        }

        let next_x = next_x as usize;
        let next_y = next_y as usize;

        if grid[next_y][next_x] == '#' {
            dir = (dir + 1) % 4; // Turn right
        } else {
            curr_x = next_x;
            curr_y = next_y;
        }
    }
}

#[test]
fn test_part1() {
    assert_eq!(part1(include_str!("../data/test/06.txt")), 42);
}

#[test]
fn test_part2() {
    assert_eq!(part2(include_str!("../data/test/06.txt")), 6);
}
