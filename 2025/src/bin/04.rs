fn main() {
    let input: &str = include_str!("../data/04.txt");
    println!("Answer to part1: {}", part1(input));
    println!("Answer to part2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let mut count = 0;
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let rows = grid.len();
    let cols = grid[0].len();
    for r in 0..rows {
        for c in 0..cols {
            // only consider rolls of paper
            if grid[r][c] != '@' {
                continue;
            }
            // check around the grid for adjacent '@'
            let mut neighbours: u8 = 0;
            for dr in -1isize..=1 {
                for dc in -1isize..=1 {
                    if dr == 0 && dc == 0 {
                        continue;
                    }
                    let new_r = r as isize + dr;
                    let new_c = c as isize + dc;
                    if new_r >= 0
                        && new_r < rows as isize
                        && new_c >= 0
                        && new_c < cols as isize
                        && grid[new_r as usize][new_c as usize] == '@'
                    {
                        neighbours += 1;
                    }
                }
            }
            if neighbours < 4 {
                count += 1;
            }
        }
    }
    count
}

fn part2(input: &str) -> usize {
    let mut add = 0;
    let mut grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let rows = grid.len();
    let cols = grid[0].len();
    let mut count = 0;
    loop {
        for r in 0..rows {
            for c in 0..cols {
                if grid[r][c] != '@' {
                    continue;
                }
                let mut neighbours: u8 = 0;
                for dr in -1isize..=1 {
                    for dc in -1isize..=1 {
                        if dr == 0 && dc == 0 {
                            continue;
                        }
                        let new_r = r as isize + dr;
                        let new_c = c as isize + dc;
                        if new_r >= 0
                            && new_r < rows as isize
                            && new_c >= 0
                            && new_c < cols as isize
                            && grid[new_r as usize][new_c as usize] == '@'
                        {
                            neighbours += 1;
                        }
                    }
                }
                if neighbours < 4 {
                    add += 1;
                    grid[r][c] = 'X';
                }
            }
        }
        if add == 0 {
            break;
        }
        count += add;
        add = 0;
    }
    count
}

#[test]
fn test_part1() {
    assert_eq!(part1(include_str!("../data/test/04.txt")), 13);
}

#[test]
fn test_part2() {
    assert_eq!(part2(include_str!("../data/test/04.txt")), 43);
}
