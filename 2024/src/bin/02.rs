fn main() {
    let input: &str = include_str!("../data/02.txt");
    println!("Answer to part1: {}", part1(input));
    println!("Answer to part2: {}", part2(input));
}

fn is_valid_sequence(seq: &[i32]) -> bool {
    let mut increasing = true;
    let mut decreasing = true;

    for i in 0..seq.len() - 1 {
        let diff = seq[i + 1] - seq[i];
        if diff.abs() < 1 || diff.abs() > 3 {
            return false;
        }
        if diff < 0 {
            increasing = false;
        }
        if diff > 0 {
            decreasing = false;
        }
    }

    increasing != decreasing
}


fn part1(input: &str) -> usize {
    let mut report: Vec<Vec<i32>> = Vec::new();
    for line in input.lines() {
        let mut row: Vec<i32> = Vec::new();
        for num in line.split_whitespace() {
            row.push(num.parse().unwrap());
        }
        report.push(row);
    }

    let mut safe_count = 0;
    for row in report {
        if is_valid_sequence(&row) {
            safe_count += 1;
        }
    }
    safe_count
}

#[test]
fn test_part1() {
    assert_eq!(part1(include_str!("../data/test/02.txt")), 2);
}

fn part2(input: &str) -> usize {
    let mut report: Vec<Vec<i32>> = Vec::new();
    for line in input.lines() {
        let mut row: Vec<i32> = Vec::new();
        for num in line.split_whitespace() {
            row.push(num.parse().unwrap());
        }
        report.push(row);
    }

    let mut safe_count = 0;

    for row in report {
        if is_valid_sequence(&row) {
            safe_count += 1;
        } else {
            for i in 0..row.len() {
                let mut new_row = row.clone();
                new_row.remove(i);
                if is_valid_sequence(&new_row) {
                    safe_count += 1;
                    break;
                }
            }
        }
    }
    safe_count
}
#[cfg(test)]
#[test]
fn test_part2() {
    assert_eq!(part2(include_str!("../data/test/02.txt")), 4);
}
