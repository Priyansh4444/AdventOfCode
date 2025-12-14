fn main() {
    let input: &str = include_str!("../data/12.txt");
    println!("Answer to part1: {}", part1(input));
    println!("Answer to part2: {}", part2(input));
}
struct Region {
    width: u32,
    length: u32,
    counts: Vec<u32>,
}

// assume all presents are 3x3
const BLOCK_SIZE: u32 = 3;


impl Region {
    fn definitely_fits(&self) -> bool {
        let blocks_free = (self.width / BLOCK_SIZE) * (self.length / BLOCK_SIZE);
        let blocks_to_place = self.counts.iter().sum();
        blocks_free >= blocks_to_place
    }
}

fn parse(input: &str) -> (Vec<Vec<Vec<bool>>>, Vec<Region>) {
    let (shapes_str, regions_str) = input.rsplit_once("\n\n").unwrap();

    let shapes = shapes_str
        .split("\n\n")
        .map(|block| {
            block
                .lines()
                .skip(1)
                .map(|line| line.chars().map(|c| c == '#').collect())
                .collect()
        })
        .collect();

    let regions = regions_str
        .lines()
        .map(|line| {
            let (tree, counts) = line.split_once(": ").unwrap();
            let (width, length) = tree.split_once("x").unwrap();
            Region {
                width: width.parse().unwrap(),
                length: length.parse().unwrap(),
                counts: counts.split(" ").map(|s| s.parse().unwrap()).collect(),
            }
        })
        .collect();

    (shapes, regions)
}

fn part1(input: &str) -> usize {
    let (_, regions) = parse(input);
    regions.into_iter().filter(Region::definitely_fits).count()
}

fn part2(_: &str) -> usize {
    0
}

#[test]
fn test_part1() {
    assert_eq!(part1(include_str!("../data/test/12.txt")), 2);
}

#[test]
fn test_part2() {
    assert_eq!(part2(include_str!("../data/test/12.txt")), 0);
}
