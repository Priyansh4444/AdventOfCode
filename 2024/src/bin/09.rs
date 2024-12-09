fn main() {
    let input: &str = &include_str!("../data/09.txt").trim().replace("\r", "");
    let now = std::time::Instant::now();
    println!("Answer to part1: {}", part1(input));
    let elapsed = now.elapsed();
    println!("Time: {:?}", elapsed);
    let now = std::time::Instant::now();
    println!("Answer to part2: {}", part2(input));
    let elapsed = now.elapsed();
    println!("Time: {:?}", elapsed);
}

fn part1(input: &str) -> usize {
    
    0
}

fn part2(input: &str) -> usize {
    
    0
}

#[test]
fn test_part1() {
    assert_eq!(part1(include_str!("../data/test/09.txt")), 0);
}

#[test]
fn test_part2() {
    assert_eq!(part2(include_str!("../data/test/09.txt")), 0);
}
