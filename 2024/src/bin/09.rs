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

// Check ben's part2 solution

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
        for _ in 0..file_size {
            disk.push(Some(file_id));
        }
        for _ in 0..free_space {
            disk.push(None);
        }
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

    let pairs = input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect::<Vec<usize>>()
        // split into (block size, free size)
        .chunks(2)
        .map(|c| (c[0], c[1]))
        .collect::<Vec<(usize, usize)>>();
    let mut disk = Vec::new();
    let mut file_sizes = Vec::new();
    let mut file_id = 0;
    let mut free_sizes = Vec::new();
    for (file_size, free_space) in pairs {
        file_sizes.push(file_size);
        free_sizes.push(free_space);
        for _ in 0..file_size {
            disk.push(Some(file_id));
        }
        for _ in 0..free_space {
            disk.push(None);
        }
        file_id += 1;
    }

    // check in decreasing order of file_id
    for current_id in (0..file_id).rev() {
        // find the size of the file (going bacwards)
        let file_size = file_sizes[current_id];
        // find the index where the file starts
        let file_start = disk.iter().position(|&x| x == Some(current_id)).unwrap();
        let mut best_pos = None;

        // Try to find if you can put a file of the biggest id, at the start of the disk or leave it at the end
        let free_spaces = disk[..file_start]
            .iter()
            .enumerate()
            .collect::<Vec<_>>() // collect to Vec to use slice methods
            .chunk_by(|(_, a), (_, b)| a.is_none() == b.is_none()) // group by None/Some
            .into_iter()
            .filter(|chunk| chunk[0].1.is_none()) // keep only None chunks
            .map(|chunk| {
                let start_pos = chunk[0].0;
                let length = chunk.len();
                (start_pos, length)
            })
            .collect::<Vec<_>>();

        for (start_pos, length) in free_spaces {
            if length >= file_size {
                best_pos = Some(start_pos);
                break;
            }
        }
        // If we found a space big enough move the fiile
        if let Some(new_pos) = best_pos {
            // Clear old position
            for i in file_start..file_start + file_size {
                disk[i] = None;
            }
            // Place at new position
            for i in new_pos..new_pos + file_size {
                disk[i] = Some(current_id);
            }
        }
    }

    // Calculate checksum
    disk.iter()
        .enumerate()
        .filter_map(|(pos, &file_id)| file_id.map(|id| pos * id))
        .sum()
}

#[test]
fn test_part1() {
    assert_eq!(part1(include_str!("../data/test/09.txt")), 1928);
}

#[test]
fn test_part2() {
    assert_eq!(part2(include_str!("../data/test/09.txt")), 2858);
}
