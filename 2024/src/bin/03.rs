fn main() {
    let input: &str = include_str!("../data/03.txt");
    println!("Answer to part1: {}", part1(input));
    println!("Answer to part2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    input
        .split("mul(")
        // example: xmul(2,4)%&mul[3,7]
        // becomes iterator over ["x", "2,4]%&mul[3,7]"]
        .skip(1)
        // skiips the first element, so we get ["2,4]%&mul[3,7]"]
        .filter_map(|s| {
            let parts: Vec<&str> = s.split(')').collect();
            if parts.len() > 1 {
                let nums: Vec<&str> = parts[0].split(',').collect();
                if nums.len() == 2 {
                    if let (Ok(a), Ok(b)) = (
                        nums[0].trim().parse::<usize>(),
                        nums[1].trim().parse::<usize>(),
                    ) {
                        return Some(a * b);
                    }
                }
            }
            None
        })
        .sum()
}

#[test]
fn test_part1() {
    assert_eq!(part1(include_str!("../data/03_test.txt")), 161);
}

fn part2(input: &str) -> usize {
    let mut switch = true;
    input
        .split("mul(")
        .skip(1)
        .filter_map(|s| {
            let parts: Vec<&str> = s.split(')').collect();
            let mut return_value = None;
            if parts.len() > 1 && switch {
                let nums: Vec<&str> = parts[0].split(',').collect();
                if nums.len() == 2 {
                    if let (Ok(a), Ok(b)) = (
                        nums[0].trim().parse::<usize>(),
                        nums[1].trim().parse::<usize>(),
                    ) {
                        return_value = Some(a * b);
                    }
                }
            } else {
                return_value = None;
            }
            let does = s.rfind("do()");
            let dont = s.rfind("don't()");
            match (does, dont) {
                (Some(does), Some(dont)) => {
                    if does > dont {
                        switch = true;
                    } else {
                        switch = false;
                    }
                }
                (Some(_), None) => {
                    switch = true;
                }
                (None, Some(_)) => {
                    switch = false;
                }

                _ => {}
            }
            return_value
        })
        .sum()
}

#[test]
fn test_part2() {
    assert_eq!(
        part2("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"),
        48
    );
}
