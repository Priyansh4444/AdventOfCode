fn main() {
    let input: &str = include_str!("../data/10.txt");
    println!("Answer to part1: {}", part1(input));
    println!("Answer to part2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let diagram = parts[0];
            let target: Vec<u8> = diagram[1..diagram.len() - 1]
                .chars()
                .map(|c| if c == '#' { 1 } else { 0 })
                .collect();
            let n = target.len();
            let mut buttons = vec![];
            for &part in &parts[1..] {
                if part.starts_with('{') {
                    break;
                }
                let nums: Vec<usize> = part[1..part.len() - 1]
                    .split(',')
                    .map(|s| s.parse().unwrap())
                    .collect();
                let mut b = vec![0u8; n];
                for &i in &nums {
                    b[i] = 1;
                }
                buttons.push(b);
            }
            // now, find min presses
            let m = buttons.len();
            let mut min_presses = usize::MAX;
            for mask in 0..(1 << m) {
                let mut current = vec![0u8; n];
                let mut presses = 0;
                for i in 0..m {
                    if (mask & (1 << i)) != 0 {
                        presses += 1;
                        for j in 0..n {
                            current[j] ^= buttons[i][j];
                        }
                    }
                }
                if current == target {
                    min_presses = min_presses.min(presses);
                }
            }
            min_presses
        })
        .sum()
}

/*
/// Initialize a slice of length `m` with the contents `[0, 0, ..., n]` and then
/// repeatedly call this function to obtain all possible combinations of `m`
/// integers that sum to `n`. The function returns `false` if there is no other
/// combination.
fn next_combination(combinations: &mut [usize]) -> bool {
    let i = combinations.iter().rposition(|&v| v != 0).unwrap();
    if i == 0 {
        return false;
    }
    let v = combinations[i];
    combinations[i - 1] += 1;
    combinations[i] = 0;
    combinations[combinations.len() - 1] = v - 1;
    true
}

/// Checks if the button at index `i` is available based on the bitmask.
fn is_button_available(i: usize, mask: u32) -> bool {
    mask & (1 << i) > 0
}

/// Part 2: Optimized DFS that tries to prune as many branches as possible.
/// This function finds the minimum number of button presses to reduce all joltage values to zero.
/// It uses a heuristic to prioritize reducing the joltage value that can be affected by the fewest buttons,
/// and prunes branches by trying combinations of presses for matching buttons.
fn dfs_part2(joltage: &[usize], available_buttons_mask: u32, buttons: &[Vec<usize>]) -> usize {
    // Base case: if all joltage values are zero, no more presses needed
    if joltage.iter().all(|j| *j == 0) {
        return 0;
    }

    // Important optimization: Find the joltage value with the lowest number of
    // combinations of buttons to try. This allows us to prune branches as early
    // as possible.
    // Second optimization (not so important, but still quite good): If multiple
    // joltage values are affected by the same number of buttons, select the
    // highest value
    let (mini, &min) = joltage
        .iter()
        .enumerate()
        .filter(|&(_, &v)| v > 0)
        .min_by_key(|&(i, &v)| {
            (
                // lowest number of buttons
                buttons
                    .iter()
                    .enumerate()
                    .filter(|&(j, b)| {
                        is_button_available(j, available_buttons_mask) && b.contains(&i)
                    })
                    .count(),
                // highest joltage value (negative because we're using `min_by_key`)
                -(v as isize),
            )
        })
        .unwrap();

    // get the buttons that affect the joltage value at position `mini`
    let matching_buttons = buttons
        .iter()
        .enumerate()
        .filter(|&(i, b)| is_button_available(i, available_buttons_mask) && b.contains(&mini))
        .collect::<Vec<_>>();

    let mut result = usize::MAX;

    if !matching_buttons.is_empty() {
        // create new mask so only those buttons remain that do not affect the
        // joltage value at position `mini`
        let mut new_mask = available_buttons_mask;
        for (i, _) in &matching_buttons {
            new_mask &= !(1 << i);
        }

        // try all combinations of matching buttons
        let mut new_joltage = joltage.to_vec();
        let mut counts = vec![0; matching_buttons.len()];
        counts[matching_buttons.len() - 1] = min;
        loop {
            // count down joltage values and make sure we don't press a button
            // too often (i.e. that the number of button presses is not higher
            // than any of the values to decrease)
            let mut good = true;
            new_joltage.copy_from_slice(joltage);
            'buttons: for (bi, &cnt) in counts.iter().enumerate() {
                if cnt == 0 {
                    continue;
                }
                for &j in matching_buttons[bi].1 {
                    if new_joltage[j] >= cnt {
                        new_joltage[j] -= cnt;
                    } else {
                        good = false;
                        break 'buttons;
                    }
                }
            }

            if good {
                // recurse with decreased joltage values and with remaining buttons
                let r = dfs_part2(&new_joltage, new_mask, buttons);
                if r != usize::MAX {
                    result = result.min(min + r);
                }
            }

            // try next combination
            if !next_combination(&mut counts) {
                break;
            }
        }
    }

    result
}

/// Solves part 2 by parsing the joltage targets from each line and using the optimized DFS
/// to compute the minimum total button presses across all machines.
fn part2(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let mut buttons = vec![];
            let mut joltage_part = "";
            // Parse buttons until we hit the joltage part
            for &part in &parts[1..] {
                if part.starts_with('{') {
                    joltage_part = part;
                    break;
                }
                let nums: Vec<usize> = part[1..part.len() - 1]
                    .split(',')
                    .map(|s| s.parse().unwrap())
                    .collect();
                buttons.push(nums);
            }
            // Parse the target joltage values
            let target_joltage: Vec<usize> = joltage_part[1..joltage_part.len() - 1]
                .split(',')
                .map(|s| s.parse().unwrap())
                .collect();
            // Compute minimum presses for this machine using DFS
            dfs_part2(&target_joltage, (1 << buttons.len()) - 1, &buttons)
        })
        .sum()
}
*/

/// Parity-pattern recursive solution for Part 2 (based on the AoC subreddit approach).
///
/// High-level idea:
/// - Group all possible button press patterns by their parity effect on each variable (light).
/// - For any target joltages, only patterns whose parity matches the target parity are relevant,
///   because pressing any button an even number of times does not change parity.
/// - For each matching pattern, ensure it does not overshoot any target component and has matching parity.
///   Then subtract the pattern from the goal and divide all components by 2, recursing.
/// - The total cost is: pattern_cost (presses in this layer) + 2 * recursive_cost (because we halved).
///
/// This runs quickly because each recursive step divides the target by 2 component-wise, shrinking
/// the problem size rapidly. We also cache results for goals we've already computed.
fn part2(input: &str) -> usize {
    // Compute the total across all lines (machines)
    input
        .lines()
        .map(|line| {
            // Parse parts: first is the diagram (ignored in part2), then zero or more button specs "(...)", then
            // the final part is the joltage goal "{...}".
            let parts: Vec<&str> = line.split_whitespace().collect();
            let goal_part = parts.last().expect("Expected a goal part");
            let goal: Vec<usize> = goal_part[1..goal_part.len() - 1]
                .split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect();
            let n = goal.len();

            // Parse buttons as coefficient vectors (0/1 per variable), i.e. which lights they affect.
            // Example: "(1,3)" becomes a length-n vector with 1s at indices 1 and 3, otherwise 0.
            let mut coeffs: Vec<Vec<usize>> = Vec::new();
            for &part in &parts[1..parts.len() - 1] {
                if !part.starts_with('(') {
                    break;
                }
                let indices: Vec<usize> = part[1..part.len() - 1]
                    .split(',')
                    .map(|s| s.parse::<usize>().unwrap())
                    .collect();
                let mut vec = vec![0usize; n];
                for &i in &indices {
                    vec[i] = 1;
                }
                coeffs.push(vec);
            }

            // Build parity-grouped pattern costs:
            // Map: parity_vector (len n, entries 0/1) -> map of full pattern vector -> minimum cost (#buttons pressed).
            //
            // A "pattern" is the sum of selected button coefficient vectors; its entry at position k
            // is how many times that variable is decreased by the selected buttons (not modulo 2).
            //
            // We enumerate all subsets of buttons; for each subset, compute:
            // - pattern: sum of coefficients
            // - parity: pattern[i] % 2 for all i
            //
            // We store the minimum cost for achieving that exact pattern in its parity bucket.
            use std::collections::HashMap;
            let b = coeffs.len();
            let mut pattern_costs: HashMap<Vec<usize>, HashMap<Vec<usize>, usize>> = HashMap::new();

            // Initialize buckets for all parity vectors of length n (2^n possibilities).
            // This avoids hashing misses later when indexing by a parity vector.
            let total_parities = 1usize << n;
            for mask in 0..total_parities {
                let mut parity = vec![0usize; n];
                for i in 0..n {
                    parity[i] = (mask >> i) & 1;
                }
                pattern_costs.insert(parity, HashMap::new());
            }

            // Enumerate all subsets of buttons to build patterns.
            // Cost is number of pressed buttons, i.e., popcount of the subset bitmask.
            for subset in 0usize..(1usize << b) {
                // Compute cost = number of buttons pressed in this subset
                let cost = subset.count_ones() as usize;

                // Sum coefficients to get the raw pattern vector
                let mut pattern = vec![0usize; n];
                for btn in 0..b {
                    if (subset & (1 << btn)) != 0 {
                        let c = &coeffs[btn];
                        for i in 0..n {
                            pattern[i] += c[i];
                        }
                    }
                }

                // Compute parity vector of the pattern
                let mut parity = vec![0usize; n];
                for i in 0..n {
                    parity[i] = pattern[i] % 2;
                }

                // Insert or update minimum cost for this exact pattern under parity bucket
                let bucket = pattern_costs.get_mut(&parity).unwrap();
                if let Some(existing) = bucket.get_mut(&pattern) {
                    if cost < *existing {
                        *existing = cost;
                    }
                } else {
                    bucket.insert(pattern, cost);
                }
            }

            // Memoization cache for recursion: goal vector -> minimal presses
            let mut cache: HashMap<Vec<usize>, usize> = HashMap::new();

            // Recursive solver per the parity-pattern approach.
            fn solve(
                goal: &[usize],
                pattern_costs: &HashMap<Vec<usize>, HashMap<Vec<usize>, usize>>,
                cache: &mut HashMap<Vec<usize>, usize>,
            ) -> usize {
                // If already solved, return cached value
                if let Some(&ans) = cache.get(goal) {
                    return ans;
                }

                // Base case: all zeros -> zero presses needed
                if goal.iter().all(|&x| x == 0) {
                    cache.insert(goal.to_vec(), 0);
                    return 0;
                }

                // Identify parity bucket for this goal
                let mut parity = vec![0usize; goal.len()];
                for i in 0..goal.len() {
                    parity[i] = goal[i] % 2;
                }

                // Large sentinel for impossible configurations
                let mut best = 1_000_000usize;

                // Consider only patterns whose parity matches the goal's parity
                if let Some(bucket) = pattern_costs.get(&parity) {
                    for (pattern, &pattern_cost) in bucket.iter() {
                        // Pattern must not overshoot any goal component
                        // and must have matching parity (already ensured by the bucket).
                        let mut ok = true;
                        for i in 0..goal.len() {
                            if pattern[i] > goal[i] || (pattern[i] % 2) != (goal[i] % 2) {
                                ok = false;
                                break;
                            }
                        }
                        if !ok {
                            continue;
                        }

                        // Build next goal by subtracting pattern and halving component-wise
                        let mut next_goal = vec![0usize; goal.len()];
                        for i in 0..goal.len() {
                            next_goal[i] = (goal[i] - pattern[i]) / 2;
                        }

                        // Recurse: total = cost now + 2 * cost later
                        let sub = solve(&next_goal, pattern_costs, cache);
                        let total = pattern_cost + 2 * sub;
                        if total < best {
                            best = total;
                        }
                    }
                }

                cache.insert(goal.to_vec(), best);
                best
            }

            // Solve this machine
            solve(&goal, &pattern_costs, &mut cache)
        })
        .sum()
}

#[test]
fn test_part1() {
    assert_eq!(part1(include_str!("../data/test/10.txt")), 7);
}

#[test]
fn test_part2() {
    assert_eq!(part2(include_str!("../data/test/10.txt")), 0);
}
