// Essentially DFS, finding shortest path while cahcing each small step on each level and then going back up recursively since we found the shortest path Downwards and returning the cost of the path
// (Caching takes care of different permuations leading back to the same permutation in coming from a different angle)
fn main() {
    let input: &str = include_str!("../data/21.txt");
    let now = std::time::Instant::now();
    println!("Answer to part1: {}", part1(input));
    let elapsed = now.elapsed();
    println!("Elapsed: {:?}", elapsed);
    let now = std::time::Instant::now();
    println!("Answer to part2: {}", part2(input));
    let elapsed = now.elapsed();
    println!("Elapsed: {:?}", elapsed);
}

use std::cmp::Ordering;
use std::collections::HashMap;

// We doing linked lists of pads going to the next pad manually everytime
#[derive(Clone)]
enum Pad {
    Person,
    Robot {
        grid: Vec<Vec<char>>,
        next: Box<Pad>,
        cache: HashMap<String, usize>,
    },
}

impl Pad {
    fn new_numpad(next: Pad) -> Self {
        let grid = "789\n456\n123\n 0A"
            .lines()
            .map(|line| line.chars().collect())
            .collect();

        Pad::Robot {
            grid,
            next: Box::new(next),
            cache: HashMap::new(),
        }
    }

    fn new_dirpad(next: Pad) -> Self {
        let grid = " ^A\n<v>"
            .lines()
            .map(|line| line.chars().collect())
            .collect();

        Pad::Robot {
            grid,
            next: Box::new(next),
            cache: HashMap::new(),
        }
    }

    fn calc_path_cost(&mut self, s: &str) -> usize {
        match self {
            Pad::Person => s.len(),
            Pad::Robot { grid, next, cache } => {
                // {"<v<A": 2414146668, "v<<A": 1970315728, "^A": 958329044 ...} Cache looks like this where the key is the small path steps and the value is the cost of the path
                if let Some(&cost) = cache.get(s) {
                    return cost;
                }
                let mut chars = s.chars();
                let first = chars.next().unwrap();
                let mut cost = calc_movement_cost(grid, next, 'A', first);
                let mut prev = first;
                for curr in chars {
                    let next = calc_movement_cost(grid, next, prev, curr);
                    cost += next;
                    prev = curr;
                }
                cache.insert(s.to_string(), cost);
                cost
            }
        }
    }
}

// Calculating the cost of the movement by recursively (DFS for the smallest movement seeing which makes the smallest movement and recursively going back)
// A->0 => go to path cost of all permuations of v<<A to all permutations of <vA and so on bascially all things tthat end with A aka every single step is cached
// on going down you cache constantly the cost of the sum of the shortest path
// on going back up you calculate the cost aka the length of the smallest path all the way down and shend it back up adding it to cache on each iteration (each small iteration is added to each cache essentially) calculating the path (it keeps going to next and gets the minimum cost of the path
fn calc_movement_cost(grid: &[Vec<char>], next: &mut Pad, from: char, to: char) -> usize {
    let paths = find_paths(grid, from, to);
    paths
        .into_iter()
        // recursively calculate the cost of the path by
        .map(|p| next.calc_path_cost(&p))
        .min()
        .unwrap()
}

// finding the path from to TO using DFS since cache better
fn find_paths(grid: &[Vec<char>], from: char, to: char) -> Vec<String> {
    let (from_pos, to_pos) = find_positions(grid, from, to);
    let dr = from_pos.0 as isize - to_pos.0 as isize;
    let dc = from_pos.1 as isize - to_pos.1 as isize;

    let mut paths = Vec::new();
    find_paths_dfs(grid, dr, dc, to_pos, String::new(), &mut paths);
    paths
}

// Finding where positions exists and returning from and to
fn find_positions(grid: &[Vec<char>], from: char, to: char) -> ((usize, usize), (usize, usize)) {
    let mut from_pos = None;
    let mut to_pos = None;

    for (i, row) in grid.iter().enumerate() {
        for (j, &c) in row.iter().enumerate() {
            if c == from {
                from_pos = Some((i, j));
            }
            if c == to {
                to_pos = Some((i, j));
            }
        }
    }

    (from_pos.unwrap(), to_pos.unwrap())
}

// This can be replaced with cartesian product of those other than the ones that go out of bounds (aka the one blank space)
fn find_paths_dfs(
    grid: &[Vec<char>],
    dr: isize,
    dc: isize,
    goal: (usize, usize),
    path: String,
    paths: &mut Vec<String>,
) {
    // Robot panics and dies and path isnt added
    if grid[(goal.0 as isize + dr) as usize][(goal.1 as isize + dc) as usize] == ' ' {
        return;
    }
    // push the button and make sure it enters paths
    if dr == 0 && dc == 0 {
        paths.push(path + "A");
        return;
    }
    // add to the path this possibility and check the next one match and add that as well
    match dr.cmp(&0) {
        Ordering::Less => find_paths_dfs(grid, dr + 1, dc, goal, path.clone() + "v", paths),
        Ordering::Greater => find_paths_dfs(grid, dr - 1, dc, goal, path.clone() + "^", paths),
        _ => {}
    }
    // same add path but column
    match dc.cmp(&0) {
        Ordering::Less => find_paths_dfs(grid, dr, dc + 1, goal, path.clone() + ">", paths),
        Ordering::Greater => find_paths_dfs(grid, dr, dc - 1, goal, path + "<", paths),
        _ => {}
    }
}

pub fn part1(input: &str) -> usize {
    let mut pad = Pad::new_numpad(Pad::new_dirpad(Pad::new_dirpad(Pad::Person)));

    input
        .lines()
        .map(|line| {
            let cost = pad.calc_path_cost(line);
            let value: usize = line[..3].parse().unwrap();
            value * cost
        })
        .sum()
}

pub fn part2(input: &str) -> usize {
    let mut pad = Pad::Person;
    for _ in 0..25 {
        pad = Pad::new_dirpad(pad);
    }
    pad = Pad::new_numpad(pad);

    input
        .lines()
        .map(|line| {
            let cost = pad.calc_path_cost(line);
            let value: usize = line[..3].parse().unwrap();
            value * cost
        })
        .sum()
}

#[test]
fn test_part1() {
    assert_eq!(part1(include_str!("../data/test/21.txt")), 126384);
}

#[test]
fn test_part2() {
    assert_eq!(part2(include_str!("../data/test/21.txt")), 154115708116294);
}
