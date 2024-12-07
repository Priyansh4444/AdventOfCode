fn main() {
    let input: &str = &include_str!("../data/07.txt").replace("\r", "");
    println!("Answer to part1: {}", part1(input));
    println!("Answer to part2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let now = std::time::Instant::now();
    let mut sum = 0;
    for line in input.lines() {
        let parts: Vec<&str> = line.split(": ").collect();
        let rhs = parts[0].parse::<usize>().unwrap();
        let lhs: Vec<usize> = parts[1]
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        if can_make_value(rhs, &lhs, false) {
            sum += rhs;
        }
    }
    println!("Time: {:?}", now.elapsed());
    sum
}

fn can_make_value(target: usize, numbers: &[usize], part2: bool) -> bool {
    if numbers.len() == 1 {
        return numbers[0] == target;
    }
    let mut ops = vec!["+", "*"];
    if part2 {
        ops.push("||");
    }
    let mut expressions = vec![numbers[0]];

    for &num in &numbers[1..] {
        let mut new_expressions = vec![];
        for val in expressions {
            for &op in &ops {
                let new_val = match op {
                    "+" => val + num,
                    "*" => val * num,
                    "||" => (val.to_string() + &num.to_string()).parse().unwrap(),
                    _ => unreachable!(),
                };
                if new_val <= target {
                    new_expressions.push(new_val);
                }
            }
        }
        expressions = new_expressions;
    }
    expressions.iter().any(|val| *val == target)
}

fn part2(input: &str) -> usize {
    let now: std::time::Instant = std::time::Instant::now();
    let mut sum = 0;
    for line in input.lines() {
        let parts: Vec<&str> = line.split(": ").collect();
        let rhs = parts[0].parse::<usize>().unwrap();
        let lhs: Vec<usize> = parts[1]
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        if can_make_value(rhs, &lhs, true) {
            sum += rhs;
        }
    }
    println!("Time: {:?}", now.elapsed());
    sum
}

#[test]
fn test_part1() {
    assert_eq!(part1(include_str!("../data/test/07.txt")), 3749);
}

#[test]
fn test_part2() {
    assert_eq!(part2(include_str!("../data/test/07.txt")), 11387);
}

// Much faster solution for part 2:

// fn part2(input: &str) -> usize {
//     let mut count = 0;
//     for line in input.lines() {
//         let (ans, right) = line.split_once(": ").unwrap();
//         let ans = ans.parse::<i64>().unwrap();
//         let nums = right
//             .split(" ")
//             .map(|v| v.parse::<i64>().unwrap())
//             .collect::<Vec<i64>>();
//         if works(nums.as_slice(), ans, true) {
//             count += ans;
//         }
//     }
//     count as usize
// }

// fn works(nums: &[i64], target: i64, part2: bool) -> bool {
//     if nums.len() == 0 {
//         return target == 0;
//     }
//     if target <= 0 {
//         return false;
//     }
//     let last = nums.last().unwrap();

//     // Multiplication case
//     if target % last == 0 {
//         if works(&nums[0..nums.len() - 1], target / last, part2) {
//             return true;
//         }
//     }

//     // Concatenation case
//     if part2 {
//         let concat_digits = (10i64).pow(last.ilog(10) + 1);
//         if (target - last) % concat_digits == 0 {
//             if works(&nums[0..nums.len() - 1], target / concat_digits, part2) {
//                 return true;
//             }
//         }
//     }

//     // Addition case
//     return works(&nums[0..nums.len() - 1], target - last, part2);
// }

// Why is it faster?
// Because you are cutting of branches that are not possible way faster
// than the other solution. The other solution is trying to calculate all
// possible ways to get to the target number, but this solution is cutting
// off branches that are not possible way faster.

// credit: @benjamin-cates