fn main() {
    let input: &str = include_str!("../data/03.txt");
    println!("Answer to part1: {}", part1(input));
    println!("Answer to part2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let mut sum = 0;
    for line in input.lines() {
        let digits: Vec<u32> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();
        let mut max_jolt = 0;
        // check every combination of two digits
        for i in 0..digits.len() {
            for j in (i + 1)..digits.len() {
                let jolt = 10 * digits[i] + digits[j];
                if jolt > max_jolt {
                    max_jolt = jolt;
                }
            }
        }
        sum += max_jolt;
    }
    sum as usize
}
fn part2(input: &str) -> usize {
    const K: usize = 12;
    let mut total: usize = 0;
    for line in input.lines() {
        let bytes = line.as_bytes();
        let n = bytes.len();
        let take = K.min(n);
        let mut start = 0;
        let mut value: usize = 0;
        for i in 0..take {
            let end_exclusive = n - (take - i) + 1;
            let max_byte = bytes[start..end_exclusive]
                .iter()
                .copied()
                .max()
                .unwrap();
            let pos = bytes[start..end_exclusive]
                .iter()
                .position(|&b| b == max_byte)
                .unwrap()
                + start;
            start = pos + 1;

            value = value * 10 + ((max_byte - b'0') as usize);
        }
        total += value;
    }
    total
}


#[test]
fn test_part1() {
    assert_eq!(part1(include_str!("../data/test/03.txt")), 357);
}

#[test]
fn test_part2() {
    assert_eq!(part2(include_str!("../data/test/03.txt")), 3121910778619);
}
