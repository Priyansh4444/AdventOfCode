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

// Check ben's part2 solution its crakced

fn part1(input: &str) -> usize {
    let input = input.to_string() + "0";
    let pairs = input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect::<Vec<usize>>()
        // split into (block size, free size)
        .chunks(2)
        .map(|c| (c[0], c[1]))
        .collect::<Vec<(usize, usize)>>();
    // each index will be the id of the file
    let mut disk = Vec::new();
    let mut file_id = 0;
    // instead of . we are using Some() and None
    for (file_size, free_space) in pairs {
        disk.extend(std::iter::repeat(Some(file_id)).take(file_size));
        disk.extend(std::iter::repeat(None).take(free_space));
        file_id += 1;
    }

    let mut left = 0;
    let mut right = disk.len() - 1;

    while left < right {
        // Find leftmost empty space
        while left < right && disk[left].is_some() {
            left += 1;
        }

        // Find rightmost file
        while right > left && disk[right].is_none() {
            right -= 1;
        }

        // If we found valid positions to swap
        if left < right && disk[left].is_none() && disk[right].is_some() {
            disk[left] = disk[right];
            disk[right] = None;
        } else {
            break;
        }
    }

    disk.iter()
        .enumerate()
        .filter_map(|(pos, &file_id)| file_id.map(|id| pos * id))
        .sum()
}

fn part2(input: &str) -> usize {
    let input = input.to_string() + "0";
    let mut pairs = vec![];
    input
        .as_bytes()
        .chunks(2)
        .enumerate()
        .for_each(|(chunk_id, a)| {
            pairs.push(((a[0] - b'0') as usize, Some(chunk_id)));
            pairs.push(((a[1] - b'0') as usize, None));
        });
    let mut i = pairs.len() - 1;
    // check in decreasing order of file_id
    while i != 0 {
        if let (size, Some(_)) = pairs[i] {
            if let Some(new_pos) = pairs
                .iter()
                .take(i)
                .position(|v| v.1 == None && v.0 >= size)
            {
                let free_size = pairs[new_pos].0;
                pairs[new_pos] = pairs[i];
                pairs[i].1 = None;
                pairs.insert(new_pos + 1, (free_size - size, None));
                continue;
            }
        }
        i -= 1;
    }

    // Calculate checksum
    pairs
        .iter()
        .map(|v| std::iter::repeat_n(v.1.unwrap_or(0), v.0))
        .flatten()
        .enumerate()
        .map(|(i, v)| i * (v as usize))
        .sum::<usize>()
}

#[test]
fn test_part1() {
    assert_eq!(part1(include_str!("../data/test/09.txt")), 1928);
}

#[test]
fn test_part2() {
    assert_eq!(part2(include_str!("../data/test/09.txt")), 2858);
}
