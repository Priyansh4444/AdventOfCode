fn main() {
    let input: &str = include_str!("../data/02.txt");
    println!("Answer to part1: {}", part1(input));
    println!("Answer to part2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let mut count = 0;
    for line in input.lines() {
        let ranges = line.split(',').collect::<Vec<&str>>();
        for range in ranges {
            if range.is_empty() {
                continue;
            }
            let bounds = range.split('-').collect::<Vec<&str>>();
            let start: usize = bounds[0].parse().unwrap();
            let end: usize = bounds[1].parse().unwrap();
            for i in start..=end {
                let number = i.to_string();
                let length = number.len();
                let first_half = &number[0..length / 2];
                let second_half = &number[length / 2..length];
                if first_half == second_half {
                    count += i;
                }
            }
        }
    }

    count
}

fn part2(input: &str) -> usize {
    fn is_invalid_id(id: usize) -> bool {
        let number = id.to_string();
        let bytes = number.as_bytes();
        let len = bytes.len();

        for pattern_len in 1..=len / 2 {
            if len % pattern_len != 0 {
                continue;
            }
            let repeats = len / pattern_len;
            if repeats < 2 {
                continue;
            }
            let pattern = &bytes[..pattern_len];
            if bytes.chunks(pattern_len).all(|chunk| chunk == pattern) {
                return true;
            }
        }
        false
    }

    let mut total = 0;

    for line in input.lines() {
        for range in line.split(',').filter(|range| !range.is_empty()) {
            let mut bounds = range.split('-');
            let (Some(start), Some(end)) = (bounds.next(), bounds.next()) else {
                continue;
            };
            let start: usize = start.parse().unwrap();
            let end: usize = end.parse().unwrap();

            for id in start..=end {
                if is_invalid_id(id) {
                    total += id;
                }
            }
        }
    }

    total
}

#[test]
fn test_part1() {
    assert_eq!(part1(include_str!("../data/test/02.txt")), 1227775554);
}

#[test]
fn test_part2() {
    assert_eq!(part2(include_str!("../data/test/02.txt")), 4174379265);
}
