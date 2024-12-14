use std::{io::Write, path::Path};

fn main() {
    let input: &str = include_str!("../data/14.txt");
    println!("Answer to part1: {}", part1(input));
    println!("Answer to part2: {}", part2(input));
}

struct Robots {
    position: (i32, i32),
    velocity: (i32, i32),
}

fn parse_input(input: &str) -> Vec<Robots> {
    let mut machines = Vec::new();

    for line in input.lines() {
        let (p, v) = line.split_once(" ").unwrap();
        let pos_str = p.trim_start_matches("p=");
        let pos_coords: Vec<i32> = pos_str.split(',').map(|x| x.parse().unwrap()).collect();

        let vel_str = v.trim_start_matches("v=");
        let vel_coords: Vec<i32> = vel_str.split(',').map(|x| x.parse().unwrap()).collect();

        machines.push(Robots {
            position: (pos_coords[0], pos_coords[1]),
            velocity: (vel_coords[0], vel_coords[1]),
        });
    }

    machines
}
fn calculate_final_position(initial_pos: i32, velocity: i32, size: i32, steps: i32) -> i32 {
    let total_movement = initial_pos + velocity * steps;
    total_movement.rem_euclid(size)
}

fn get_final_positions(robots: &[Robots], width: i32, height: i32, steps: i32) -> Vec<(i32, i32)> {
    robots
        .iter()
        .map(|robot| {
            let final_x =
                calculate_final_position(robot.position.0, robot.velocity.0, width, steps);
            let final_y =
                calculate_final_position(robot.position.1, robot.velocity.1, height, steps);
            (final_x, final_y)
        })
        .collect()
}

fn count_robots_in_quadrants(
    positions: &[(i32, i32)],
    width: i32,
    height: i32,
) -> (usize, usize, usize, usize) {
    let mid_x = width / 2;
    let mid_y = height / 2;

    let mut counts = (0, 0, 0, 0);
    for &(x, y) in positions {
        // Skip robots on the middle lines
        if x == mid_x || y == mid_y {
            continue;
        }

        match (x < mid_x, y < mid_y) {
            (true, true) => counts.0 += 1,   // top-left
            (false, true) => counts.1 += 1,  // top-right
            (true, false) => counts.2 += 1,  // bottom-left
            (false, false) => counts.3 += 1, // bottom-right
        }
    }

    counts
}

fn part1(input: &str) -> usize {
    let robots = parse_input(input);
    let width = 101;
    let height = 103;

    // Calculate final positions after 100 steps directly
    let final_positions: Vec<(i32, i32)> = get_final_positions(&robots, width, height, 100);

    // Get quadrant counts and multiply them together
    let (q1, q2, q3, q4) = count_robots_in_quadrants(&final_positions, width, height);
    q1 * q2 * q3 * q4
}
fn update_grid(positions: &[(i32, i32)], width: i32, height: i32) -> Vec<Vec<&'static str>> {
    let mut grid = vec![vec!["."; width as usize]; height as usize];
    for &(x, y) in positions {
        if x >= 0 && x < width && y >= 0 && y < height {
            grid[y as usize][x as usize] = "#";
        }
    }
    grid
}

fn has_hashtag_line(grid: &Vec<Vec<&str>>) -> bool {
    // Check each row for a consecutive line of hashtags
    for row in grid {
        let mut consecutive_count = 0;
        let mut max_consecutive = 0;

        for &cell in row {
            if cell == "#" {
                consecutive_count += 1;
                max_consecutive = max_consecutive.max(consecutive_count);
            } else {
                consecutive_count = 0;
            }
        }

        // You can adjust this number based on how long of a line you want to detect
        if max_consecutive >= 5 {
            // Assuming we want at least 5 consecutive #s
            return true;
        }
    }
    false
}

fn part2(input: &str) -> usize {
    let robots = parse_input(input);
    let width = 101;
    let height = 103;
    let mut time = 0;
    let path = Path::new("robot_patterns.txt");
    let mut file = std::fs::File::create(path).unwrap();
    // Check positions every second until we find a Christmas tree pattern
    loop {
        if time % 100 == 0 {
            println!("Checking time: {}", time);
        }

        let positions = get_final_positions(&robots, width, height, time);
        let grid = update_grid(&positions, width, height);
        if has_hashtag_line(&grid) {
            file.write_all(format!("Time step: {}\n", time).as_bytes());

            // Write each row of the grid
            for row in grid {
                file.write_all(format!("{}\n", row.join("")).as_bytes());
            }

            // Write an empty line for separation
            file.write_all(b"\n");
        }
        // Since the motion is periodic, we can calculate the period
        // based on the grid dimensions and velocities to avoid infinite loops
        if time > 10000 {
            // Reasonable upper limit
            println!("No pattern found within reasonable time");
            return 0;
        }

        time += 1;
    }
}


#[test]
fn test_part1() {
    assert_eq!(part1(include_str!("../data/test/14.txt")), 21);
}

#[test]
fn test_part2() {
    assert_eq!(part2(include_str!("../data/test/14.txt")), 0);
}
