use std::collections::{HashMap, HashSet};

fn main() {
    let input: &str = include_str!("../data/08.txt");
    let now = std::time::Instant::now();
    println!("Answer to part1: {}", part1(input));
    println!("Time: {:?}", now.elapsed());
    let now = std::time::Instant::now();
    println!("Answer to part2: {}", part2(input));
    println!("Time: {:?}", now.elapsed());
}

#[derive(Debug)]
struct Antenna {
    x: i32,
    y: i32,
    frequency: char,
}

fn find_antennas(input: &str) -> Vec<Antenna> {
    input
        .lines()
        .enumerate()
        .fold(Vec::new(), |mut acc, (y, line)| {
            for (x, c) in line.chars().enumerate() {
                if c != '.' {
                    acc.push(Antenna {
                        x: x as i32,
                        y: y as i32,
                        frequency: c,
                    });
                }
            }
            acc
        })
}

fn calculate_antinodes(
    ant1: &Antenna,
    ant2: &Antenna,
    width: i32,
    height: i32,
    grid: &mut Vec<Vec<char>>,
) -> Vec<(i32, i32)> {
    let mut nodes = Vec::new();
    let dx = ant2.x - ant1.x;
    let dy = ant2.y - ant1.y;
    let node1_x = ant1.x + 2 * dx;
    let node1_y = ant1.y + 2 * dy;
    let node2_x = ant2.x - 2 * dx;
    let node2_y = ant2.y - 2 * dy;
    if node1_x >= 0 && node1_x < width && node1_y >= 0 && node1_y < height {
        nodes.push((node1_x, node1_y));
    }
    if node2_x >= 0 && node2_x < width && node2_y >= 0 && node2_y < height {
        nodes.push((node2_x, node2_y));
    }
    for (x, y) in &nodes {
        grid[*y as usize][*x as usize] = '#';
    }
    nodes
}

fn part1(input: &str) -> usize {
    let height = input.lines().count() as i32;
    let width = input.lines().next().unwrap().len() as i32;
    let mut grid = vec![vec!['.'; width as usize]; height as usize];
    let antennas = find_antennas(input);
    let mut freq_groups: HashMap<char, Vec<&Antenna>> = HashMap::new();
    for ant in &antennas {
        freq_groups.entry(ant.frequency).or_default().push(ant);
    }

    let mut antinode_positions = HashSet::new();

    for antennas in freq_groups.values() {
        for i in 0..antennas.len() {
            for j in (i + 1)..antennas.len() {
                let nodes = calculate_antinodes(antennas[i], antennas[j], width, height, &mut grid);
                antinode_positions.extend(nodes);
            }
        }
    }
    for line in grid {
        println!("{}", line.iter().collect::<String>());
    }
    antinode_positions.len()
}

fn part2(input: &str) -> usize {
    let height = input.lines().count() as i32;
    let width = input.lines().next().unwrap().len() as i32;
    let mut grid = vec![vec!['.'; width as usize]; height as usize];
    let antennas = find_antennas(input);
    let mut freq_groups: HashMap<char, Vec<&Antenna>> = HashMap::new();
    for ant in &antennas {
        freq_groups.entry(ant.frequency).or_default().push(ant);
    }
    let mut antinode_positions = HashSet::new();

    // For each frequency groups
    for antennas in freq_groups.values() {
        for i in 0..antennas.len() {
            for j in (i + 1)..antennas.len() {
                let nodes =
                    calculate_antinodes_2(antennas[i], antennas[j], width, height, &mut grid);
                antinode_positions.extend(nodes);
            }
        }
    }
    for line in grid {
        println!("{}", line.iter().collect::<String>());
    }
    antinode_positions.len()
}

fn calculate_antinodes_2(
    ant1: &Antenna,
    ant2: &Antenna,
    width: i32,
    height: i32,
    grid: &mut Vec<Vec<char>>,
) -> Vec<(i32, i32)> {
    let mut nodes = Vec::new();
    let dx = ant2.x - ant1.x;
    let dy = ant2.y - ant1.y;
    let mut x = ant1.x;
    let mut y = ant1.y;
    while x >= 0 && x < width && y >= 0 && y < height {
        nodes.push((x, y));
        x += dx;
        y += dy;
    }
    x = ant1.x - dx;
    y = ant1.y - dy;
    while x >= 0 && x < width && y >= 0 && y < height {
        nodes.push((x, y));
        x -= dx;
        y -= dy;
    }
    for (x, y) in &nodes {
        grid[*y as usize][*x as usize] = '#';
    }
    nodes
}

#[test]
fn test_part1() {
    let now = std::time::Instant::now();
    assert_eq!(part1(include_str!("../data/test/08.txt")), 14);
    println!("Time: {:?}", now.elapsed());
}

#[test]
fn test_part2() {
    let now = std::time::Instant::now();
    assert_eq!(part2(include_str!("../data/test/08.txt")), 34);
    println!("Time: {:?}", now.elapsed());
}
