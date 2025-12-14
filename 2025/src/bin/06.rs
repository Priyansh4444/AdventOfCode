fn main() {
    let input: &str = include_str!("../data/06.txt");
    println!("Answer to part1: {}", part1(input));
    println!("Answer to part2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let mut rows: Vec<Vec<&str>> = Vec::new();
    for line in input.lines() {
        let tokens: Vec<&str> = line.split_whitespace().collect();
        if !tokens.is_empty() {
            rows.push(tokens);
        }
    }
    let last_idx = rows.len() - 1;
    let operators = &rows[last_idx];

    let cols = rows[0].len();

    let mut total_result: usize = 0;

    for col in 0..cols {
        let mut values: Vec<usize> = Vec::new();
        for r in 0..last_idx {
            if col < rows[r].len() {
                if let Ok(v) = rows[r][col].parse::<usize>() {
                    values.push(v);
                }
            }
        }

        let op = operators[col];
        let col_result = match op {
            "*" => values
                .iter()
                .copied()
                .fold(1usize, |acc, x| acc.saturating_mul(x)),
            "+" => values.iter().copied().sum::<usize>(),
            _ => values.iter().copied().sum::<usize>(),
        };

        total_result += col_result;
    }

    total_result
}
fn part2(input: &str) -> usize {
    // Read all lines and determine the maximum width
    let lines: Vec<&str> = input.lines().collect();
    let line_count = lines.len();
    let max_width = lines.iter().map(|l| l.len()).max().unwrap_or(0);

    // Build columns: for each column index, collect the character in each row (pad with space if missing)
    let mut cols: Vec<Vec<char>> = Vec::with_capacity(max_width);
    for c in 0..max_width {
        let mut col_vec: Vec<char> = Vec::with_capacity(line_count);
        for r in 0..line_count {
            let ch = lines[r].chars().nth(c).unwrap_or(' ');
            col_vec.push(ch);
        }
        cols.push(col_vec);
    }

    // Helper to detect a full-space separator column
    let is_separator = |col: &[char]| col.iter().all(|&ch| ch == ' ');

    // Process right-to-left, grouping columns into problems separated by full-space columns
    let mut total_result: usize = 0;
    let mut current_group: Vec<usize> = Vec::new();

    let finalize_group = |group: &Vec<usize>| {
        if group.is_empty() {
            return 0usize;
        }

        // Find operator in the bottom row across the group's columns
        let mut op: Option<char> = None;
        for &ci in group.iter() {
            let bottom = cols[ci][line_count - 1];
            if bottom == '+' || bottom == '*' {
                op = Some(bottom);
                break;
            }
        }

        // Build numbers: each column is a number with digits from top..bottom (exclude bottom operator row)
        let mut values: Vec<usize> = Vec::new();
        for &ci in group.iter() {
            let mut s = String::new();
            for r in 0..(line_count - 1) {
                let ch = cols[ci][r];
                if ch.is_ascii_digit() {
                    s.push(ch);
                }
            }
            if !s.is_empty() {
                if let Ok(v) = s.parse::<usize>() {
                    values.push(v);
                }
            }
        }

        if values.is_empty() {
            return 0usize;
        }

        match op.unwrap_or('+') {
            '*' => values
                .iter()
                .copied()
                .fold(1usize, |acc, x| acc.saturating_mul(x)),
            '+' => values.iter().copied().sum::<usize>(),
            _ => values.iter().copied().sum::<usize>(),
        }
    };

    // Iterate columns from right to left
    let mut c = max_width as isize - 1;
    while c >= 0 {
        let idx = c as usize;
        if is_separator(&cols[idx]) {
            // Separator: finalize current group if any
            total_result += finalize_group(&current_group);
            current_group.clear();
        } else {
            // Part of current problem
            current_group.push(idx);
        }
        c -= 1;
    }
    // Finalize any remaining group at the left edge
    total_result += finalize_group(&current_group);

    total_result
}

#[test]
fn test_part1() {
    assert_eq!(part1(include_str!("../data/test/06.txt")), 4277556);
}

#[test]
fn test_part2() {
    assert_eq!(part2(include_str!("../data/test/06.txt")), 3263827);
}
