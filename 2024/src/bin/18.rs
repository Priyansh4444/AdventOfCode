use std::collections::{HashSet, VecDeque};

fn main() {
    let input: &str = include_str!("../data/18.txt");
    println!("Answer to part1: {}", part1(input));
    println!("Answer to part2: {:?}", part2(input));
}

fn parse_coordinates(input: &str, limit: usize) -> HashSet<(usize, usize)> {
    input
        .lines()
        .take(limit) // Only take the first 'limit' coordinates
        .map(|line| {
            let coords: Vec<usize> = line.split(',').map(|x| x.trim().parse().unwrap()).collect();
            (coords[0], coords[1])
        })
        .collect()
}

fn find_shortest_path(
    corrupted: &HashSet<(usize, usize)>,
    width: usize,
    height: usize,
) -> Option<usize> {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    queue.push_back(((0, 0), 0));
    visited.insert((0, 0));

    while let Some(((x, y), steps)) = queue.pop_front() {
        if x == width - 1 && y == height - 1 {
            return Some(steps);
        }

        for (dx, dy) in directions {
            let new_x = x as i32 + dx;
            let new_y = y as i32 + dy;

            if new_x >= 0 && new_y >= 0 {
                let new_x = new_x as usize;
                let new_y = new_y as usize;

                if new_x < width
                    && new_y < height
                    && !corrupted.contains(&(new_x, new_y))
                    && !visited.contains(&(new_x, new_y))
                {
                    visited.insert((new_x, new_y));
                    queue.push_back(((new_x, new_y), steps + 1));
                }
            }
        }
    }
    None
}

fn part1(input: &str) -> usize {
    let width = 71;
    let height = 71;
    let corrupted = parse_coordinates(input, 1024);
    find_shortest_path(&corrupted, width, height).unwrap_or(1000)
}

fn part2(input: &str) -> (usize, usize) {
    let width = 71;
    let height = 71;
    let all_coords: Vec<(usize, usize)> = input
        .lines()
        .map(|line| {
            let coords: Vec<usize> = line.split(',').map(|x| x.trim().parse().unwrap()).collect();
            (coords[0], coords[1])
        })
        .collect();

    let mut left = 1;
    let mut right = all_coords.len();
    let mut last_working = 0;

    while left <= right {
        let mid = (left + right) / 2;
        let corrupted: HashSet<_> = all_coords.iter().take(mid).cloned().collect();

        match find_shortest_path(&corrupted, width, height) {
            Some(_) => {
                left = mid + 1;
                last_working = mid;
            }
            None => {
                right = mid - 1;
            }
        }
    }

    // The critical byte is the one after last_working
    all_coords[last_working]
}

#[test]
fn test_part1() {
    assert_eq!(part1(include_str!("../data/test/18.txt")), 0);
}

#[test]
fn test_part2() {
    assert_eq!(part2(include_str!("../data/test/18.txt")), (0, 0));
}
