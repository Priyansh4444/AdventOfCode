fn main() {
    let input: &str = include_str!("../data/08.txt");
    let now = std::time::Instant::now();
    println!("Answer to part1: {}", part1(input));
    println!("Time: {:?}", now.elapsed());
    let now = std::time::Instant::now();
    println!("Answer to part2: {}", part2(input));
    println!("Time: {:?}", now.elapsed());
}

fn part1(input: &str) -> usize {
    0
}

fn part2(input: &str) -> usize {
    0
}

#[test]
fn test_part1() {
    let now = std::time::Instant::now();
    assert_eq!(part1(include_str!("../data/test/08.txt")), 0);
    println!("Time: {:?}", now.elapsed());
}

#[test]
fn test_part2() {
    let now = std::time::Instant::now();
    assert_eq!(part2(include_str!("../data/test/08.txt")), 0);
    println!("Time: {:?}", now.elapsed());
}
