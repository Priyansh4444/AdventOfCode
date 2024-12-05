fn main() {
    let input: &str = include_str!("../data/18.txt");
    println!("Answer to part1: {}", part1(input));
    println!("Answer to part2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    0
}

fn part2(input: &str) -> usize {
    0
}

#[test]
fn test_part1() {
    assert_eq!(part1(include_str!("../data/test/18.txt")), 0);
}

#[test]
fn test_part2() {
    assert_eq!(part2(include_str!("../data/test/18.txt")), 0);
}
