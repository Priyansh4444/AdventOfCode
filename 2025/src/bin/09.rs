fn main() {
    let input: &str = include_str!("../data/09.txt");
    println!("Answer to part1: {}", part1(input));
    println!("Answer to part2: {}", part2(input));
}

struct Point {
    x: i32,
    y: i32,
}

use std::collections::{BTreeSet, HashSet, VecDeque};

fn area(p1: &Point, p2: &Point) -> u64 {
    let width = (p1.x - p2.x).abs() + 1;
    let height = (p1.y - p2.y).abs() + 1;
    (width as u64) * (height as u64)
}

fn part1(input: &str) -> usize {
    let points: Vec<Point> = input
        .lines()
        .map(|line| {
            let coords: Vec<i32> = line
                .split(',')
                .map(|num| num.parse::<i32>().unwrap())
                .collect();
            Point {
                x: coords[0],
                y: coords[1],
            }
        })
        .collect();

    let num_points = points.len();
    let mut max_area = 0;
    for i in 0..num_points {
        for j in 0..num_points {
            if i != j {
                let area_ij = area(&points[i], &points[j]);
                if area_ij > max_area {
                    max_area = area_ij;
                }
            }
        }
    }
    max_area as usize
}

fn part2(input: &str) -> usize {
    let points: Vec<Point> = input
        .lines()
        .map(|line| {
            let coords: Vec<i32> = line
                .split(',')
                .map(|num| num.parse::<i32>().unwrap())
                .collect();
            Point {
                x: coords[0],
                y: coords[1],
            }
        })
        .collect();

    // get all x's and y's
    let mut xs_set: BTreeSet<i32> = BTreeSet::new();
    let mut ys_set: BTreeSet<i32> = BTreeSet::new();
    for p in &points {
        xs_set.insert(p.x);
        ys_set.insert(p.y);
    }
    // B-TreeSet to Vec, should be sorted
    let xs: Vec<i32> = xs_set.into_iter().collect();
    let ys: Vec<i32> = ys_set.into_iter().collect();
    //  Make a grid of size (no. of x;s ) to store points(*2) to store tha gap between them for compression (-1) last row doesnt need gap
    let grid_height = xs.len() * 2 - 1;
    let grid_width = ys.len() * 2 - 1;
    let mut grid: Vec<Vec<i32>> = vec![vec![0; grid_width]; grid_height];

    // Path Compression
    for i in 0..points.len() {
        let p1 = &points[i];
        let p2 = &points[(i + 1) % points.len()];
        let cx1 = xs.iter().position(|&x| x == p1.x).unwrap() * 2;
        let cx2 = xs.iter().position(|&x| x == p2.x).unwrap() * 2;
        let cy1 = ys.iter().position(|&y| y == p1.y).unwrap() * 2;
        let cy2 = ys.iter().position(|&y| y == p2.y).unwrap() * 2;
        let cx_min = cx1.min(cx2);
        let cx_max = cx1.max(cx2);
        let cy_min = cy1.min(cy2);
        let cy_max = cy1.max(cy2);
        for cx in cx_min..=cx_max {
            for cy in cy_min..=cy_max {
                grid[cx][cy] = 1;
            }
        }
    }

    let mut outside: HashSet<(i32, i32)> = HashSet::new();
    outside.insert((-1, -1));
    let mut queue: VecDeque<(i32, i32)> = VecDeque::new();
    queue.push_back((-1, -1));
    // flood fill to find outside area, mark them in outside set
    while let Some((tx, ty)) = queue.pop_front() {
        for (nx, ny) in [(tx - 1, ty), (tx + 1, ty), (tx, ty - 1), (tx, ty + 1)] {
            if nx < -1 || ny < -1 || nx > grid_height as i32 || ny > grid_width as i32 {
                continue;
            }
            if nx >= 0
                && nx < grid_height as i32
                && ny >= 0
                && ny < grid_width as i32
                && grid[nx as usize][ny as usize] == 1
            {
                continue;
            }
            if outside.contains(&(nx, ny)) {
                continue;
            }
            outside.insert((nx, ny));
            queue.push_back((nx, ny));
        }
    }
    // fill everythign inside the grid now
    for x in 0..grid_height {
        for y in 0..grid_width {
            if !outside.contains(&(x as i32, y as i32)) {
                grid[x][y] = 1;
            }
        }
    }

    //After filling the inside area, build prefix sum array (this is pretty hard to get right)
    let mut psa: Vec<Vec<i32>> = vec![vec![0; grid_width]; grid_height];
    for x in 0..grid_height {
        for y in 0..grid_width {
            let left = if x > 0 { psa[x - 1][y] } else { 0 };
            let top = if y > 0 { psa[x][y - 1] } else { 0 };
            let topleft = if x > 0 && y > 0 { psa[x - 1][y - 1] } else { 0 };
            psa[x][y] = left + top - topleft + grid[x][y];
        }
    }

    let valid = |x1: i32, y1: i32, x2: i32, y2: i32| -> bool {
        // compress coordinates. Then query the psa to see if all cells in the rectangle are filled
        let cx1 = xs.iter().position(|&x| x == x1).unwrap() * 2;
        let cx2 = xs.iter().position(|&x| x == x2).unwrap() * 2;
        let cy1 = ys.iter().position(|&y| y == y1).unwrap() * 2;
        let cy2 = ys.iter().position(|&y| y == y2).unwrap() * 2;
        let cx_min = cx1.min(cx2);
        let cx_max = cx1.max(cx2);
        let cy_min = cy1.min(cy2);
        let cy_max = cy1.max(cy2);
        // cancellation on left
        let left = if cx_min > 0 {
            psa[cx_min - 1][cy_max]
        } else {
            0
        };
        // cancellation on top
        let top = if cy_min > 0 {
            psa[cx_max][cy_min - 1]
        } else {
            0
        };
        // add back topleft (since cancelled twice)
        let topleft = if cx_min > 0 && cy_min > 0 {
            psa[cx_min - 1][cy_min - 1]
        } else {
            0
        };
        // total count in rectangle
        let count = psa[cx_max][cy_max] - left - top + topleft;
        let total = (cx_max - cx_min + 1) * (cy_max - cy_min + 1);
        count == total as i32
    };

    let mut max_area = 0;
    for i in 0..points.len() {
        for j in 0..i {
            let p1 = &points[i];
            let p2 = &points[j];
            if valid(p1.x, p1.y, p2.x, p2.y) {
                let area = ((p1.x - p2.x).abs() + 1) * ((p1.y - p2.y).abs() + 1);
                if area > max_area {
                    max_area = area;
                }
            }
        }
    }
    max_area as usize
}

#[test]
fn test_part1() {
    assert_eq!(part1(include_str!("../data/test/09.txt")), 50);
}

#[test]
fn test_part2() {
    assert_eq!(part2(include_str!("../data/test/09.txt")), 0);
}
