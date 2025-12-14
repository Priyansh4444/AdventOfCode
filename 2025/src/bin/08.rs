fn main() {
    let now = std::time::Instant::now();
    let input: &str = include_str!("../data/08.txt");
    println!("Answer to part1: {}", part1(input, 1000));
    let elapsed = now.elapsed();
    println!("Time elapsed is: {:.2?}", elapsed);
    println!("Answer to part2: {}", part2(input));
    let elapsed = now.elapsed();
    println!("Time elapsed is: {:.2?}", elapsed);
}

use std::cmp::Reverse;

struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl Point {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Point { x, y, z }
    }

    fn distance(&self, other: &Point) -> i64 {
        let dx = (self.x - other.x) as i64;
        let dy = (self.y - other.y) as i64;
        let dz = (self.z - other.z) as i64;
        dx * dx + dy * dy + dz * dz
    }
}

fn parse_points(input: &str) -> Vec<Point> {
    input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|line| {
            let mut parts = line.split(',');
            let x = parts.next().unwrap().trim().parse::<i32>().unwrap();
            let y = parts.next().unwrap().trim().parse::<i32>().unwrap();
            let z = parts.next().unwrap().trim().parse::<i32>().unwrap();
            Point::new(x, y, z)
        })
        .collect()
}

fn part1(input: &str, edges_count: usize) -> usize {
    let points = parse_points(input);
    let n = points.len();

    // Build all pairwise edges: (distance, i, j)
    let mut edges: Vec<(i64, usize, usize)> = Vec::new();
    for i in 0..n {
        for j in (i + 1)..n {
            let d = points[i].distance(&points[j]);
            edges.push((d, i, j));
        }
    }

    edges.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    // Each Junction Box starts in its own component, size 1
    let mut comp_id: Vec<usize> = (0..n).collect();
    let mut comp_size: Vec<usize> = vec![1; n];

    // How many connections to make (1000 for the problem, 10 for the test)
    let k = edges_count;

    for idx in 0..k {
        let (_, a, b) = edges[idx];
        let ca = comp_id[a];
        let cb = comp_id[b];
        if ca == cb {
            // if two junctions are already connected by the same component, skip
            continue;
        }
        let keep = ca;
        let drop = cb;
        // update the parent node of all nodes in the dropped component
        for i in 0..n {
            if comp_id[i] == drop {
                comp_id[i] = keep;
            }
        }
        // for the problem how big is each comopnent
        comp_size[keep] += comp_size[drop];
        comp_size[drop] = 0;
    }

    let mut sizes: Vec<usize> = comp_size.into_iter().filter(|&s| s > 0).collect();
    sizes.sort_unstable_by_key(|&s| Reverse(s));
    sizes[0] * sizes[1] * sizes[2]
}

fn part2(input: &str) -> i64 {
    let points = parse_points(input);
    let n = points.len();
    if n == 0 {
        return 0;
    }
    let mut edges: Vec<(i64, usize, usize)> = Vec::new();
    for i in 0..n {
        for j in (i + 1)..n {
            let d2 = points[i].distance(&points[j]);
            edges.push((d2, i, j));
        }
    }

    edges.sort_unstable_by_key(|e| e.0);

    let mut comp_id: Vec<usize> = (0..n).collect();
    let mut comp_size: Vec<usize> = vec![1; n];
    let mut components_remaining = n;

    let mut last_a: usize = 0;
    let mut last_b: usize = 0;

    for &(_, a, b) in &edges {
        let ca = comp_id[a];
        let cb = comp_id[b];

        if ca == cb {
            continue;
        }

        last_a = a;
        last_b = b;

        let keep = ca;
        let drop = cb;

        for i in 0..n {
            if comp_id[i] == drop {
                comp_id[i] = keep;
            }
        }

        comp_size[keep] += comp_size[drop];
        comp_size[drop] = 0;

        components_remaining -= 1;
        if components_remaining == 1 {
            break;
        }
    }
    // last two component how big are they
    let xa = points[last_a].x as i64;
    let xb = points[last_b].x as i64;
    xa * xb
}

#[test]
fn test_part1() {
    assert_eq!(part1(include_str!("../data/test/08.txt"), 10), 40);
}

#[test]
fn test_part2() {
    assert_eq!(part2(include_str!("../data/test/08.txt")), 25272);
}
