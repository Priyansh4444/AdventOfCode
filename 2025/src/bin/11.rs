fn main() {
    let input: &str = include_str!("../data/11.txt");
    println!("Answer to part1: {}", part1(input));
    println!("Answer to part2: {}", part2(input));
}
use std::collections::HashMap;

fn part1(input: &str) -> usize {
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    for line in input.lines() {
        let parts: Vec<&str> = line.split(": ").collect();
        let name = parts[0].to_string();
        let outputs: Vec<String> = parts[1].split_whitespace().map(|s| s.to_string()).collect();
        graph.insert(name, outputs);
    }

    fn dfs(current: &str, target: &str, graph: &HashMap<String, Vec<String>>) -> usize {
        if current == target {
            return 1;
        }
        let mut count = 0;
        if let Some(outputs) = graph.get(current) {
            for output in outputs {
                count += dfs(output, target, graph);
            }
        }
        count
    }

    dfs("you", "out", &graph)
}
fn part2(input: &str) -> usize {
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    for line in input.lines() {
        let parts: Vec<&str> = line.split(": ").collect();
        let name = parts[0].to_string();
        let outputs: Vec<String> = parts[1].split_whitespace().map(|s| s.to_string()).collect();
        graph.insert(name, outputs);
    }

    fn dfs(
        current: &str,
        target: &str,
        graph: &HashMap<String, Vec<String>>,
        mut seen_dac: bool,
        mut seen_fft: bool,
        memo: &mut HashMap<(String, bool, bool), usize>,
    ) -> usize {
        if current == "dac" {
            seen_dac = true;
        }
        if current == "fft" {
            seen_fft = true;
        }

        let key = (current.to_string(), seen_dac, seen_fft);
        if let Some(&cached) = memo.get(&key) {
            return cached;
        }

        if current == target {
            let res = if seen_dac && seen_fft { 1 } else { 0 };
            memo.insert(key, res);
            return res;
        }

        let mut count = 0;
        if let Some(outputs) = graph.get(current) {
            for output in outputs {
                count += dfs(output, target, graph, seen_dac, seen_fft, memo);
            }
        }

        memo.insert(key, count);
        count
    }

    let mut memo: HashMap<(String, bool, bool), usize> = HashMap::new();
    dfs("svr", "out", &graph, false, false, &mut memo)
}

#[test]
fn test_part1() {
    assert_eq!(part1(include_str!("../data/test/11.txt")), 5);
}

#[test]
fn test_part2() {
    assert_eq!(part2(include_str!("../data/test/11.txt")), 2);
}
