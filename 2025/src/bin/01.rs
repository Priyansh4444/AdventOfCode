fn main() {
    let input: &str = include_str!("../data/01.txt");
    println!("Answer to part1: {}", part1(input));
    println!("Answer to part2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let mut count = 0;
    let mut current: i32 = 50;
    for line in input.lines() {
        let direction = &line[0..1];
        let rotation: i32 = line[1..].parse().unwrap();
        if direction == "L" {
            current = (current - rotation).rem_euclid(100);
        } else {
            current = (current + rotation).rem_euclid(100);
        }
        if current == 0 {
            count += 1;
        }
    }
    count
}

#[test]
fn test_part1() {
    assert_eq!(part1(include_str!("../data/test/01.txt")), 3);
}

fn part2(input: &str) -> usize {
    fn hits_for_rotation(start: i32, rotation: i32, direction: i32) -> usize {
        let mut steps_till_0_based_on_direction = if direction == 1 {
            (100 - (start.rem_euclid(100))) % 100 // moving right
        } else {
            (start.rem_euclid(100)) % 100 // moving left
        };
        // first_k is the
        if steps_till_0_based_on_direction == 0 {
            steps_till_0_based_on_direction = 100;
        }
        if rotation >= steps_till_0_based_on_direction {
            (1 + (rotation - steps_till_0_based_on_direction) / 100) as usize
        } else {
            0
        }
    }

    let mut count: usize = 0;
    let mut current: i32 = 50;
    for line in input.lines() {
        let direction = &line[0..1];
        let rotation: i32 = line[1..].parse().unwrap();
        let delta = if direction == "L" { -1 } else { 1 };
        count += hits_for_rotation(current, rotation, delta);
        current = (current + delta * rotation).rem_euclid(100);
    }
    count
}

#[cfg(test)]
#[test]
fn test_part2() {
    assert_eq!(part2(include_str!("../data/test/01.txt")), 6);
}
