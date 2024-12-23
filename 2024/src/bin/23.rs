fn main() {
    let input: &str = include_str!("../data/23.txt");
    println!("Answer to part1: {}", part1(input));
    println!("Answer to part2: {}", part2(input));
}
use std::collections::{HashMap, HashSet};

fn build_graph(input: &str) -> HashMap<String, HashSet<String>> {
    let mut graph = HashMap::new();
    for line in input.lines() {
        let (a, b) = line.split_once('-').unwrap();
        graph
            .entry(a.to_string())
            .or_insert_with(HashSet::new)
            .insert(b.to_string());
        graph
            .entry(b.to_string())
            .or_insert_with(HashSet::new)
            .insert(a.to_string());
    }

    graph
}

fn find_triangles(graph: &HashMap<String, HashSet<String>>) -> HashSet<Vec<String>> {
    let mut triangles = HashSet::new();

    for (current_node, connected_nodes) in graph {
        // For each pair of neighbors
        for first_neighbor in connected_nodes {
            for second_neighbor in connected_nodes {
                // Avoid duplicates by enforcing order
                if first_neighbor <= second_neighbor {
                    continue;
                }

                // Check if neighbors are connected to each other
                if !graph[first_neighbor].contains(second_neighbor) {
                    continue;
                }

                // Found a triangle: current_node <-> first_neighbor <-> second_neighbor
                let mut triangle = vec![
                    current_node.clone(),
                    first_neighbor.clone(),
                    second_neighbor.clone(),
                ];
                triangle.sort();
                triangles.insert(triangle);
            }
        }
    }

    triangles
}

fn part1(input: &str) -> usize {
    let graph = build_graph(input);
    let triangles = find_triangles(&graph);

    triangles
        .iter()
        .filter(|triangle| triangle.iter().any(|name| name.starts_with('t')))
        .count()
}

fn find_lan_party(graph: &HashMap<String, HashSet<String>>) -> Vec<String> {
    // all computers
    let nodes: Vec<&String> = graph.keys().collect();
    // computer to the index in the list
    let node_to_idx: HashMap<&String, usize> =
        nodes.iter().enumerate().map(|(i, &n)| (n, i)).collect();

    // adjacency matrix representing the connections between computers
    let n = nodes.len();
    let mut adjacency_matrix = vec![vec![false; n]; n];
    for (u, neighbors) in graph {
        let u_idx = node_to_idx[u];
        for v in neighbors {
            let v_idx = node_to_idx[v];
            adjacency_matrix[u_idx][v_idx] = true;
            adjacency_matrix[v_idx][u_idx] = true;
        }
    }

    // sorting by whoever has the most direct connections
    let mut degrees: Vec<(usize, &&String)> = nodes.iter().map(|n| (graph[*n].len(), n)).collect();
    degrees.sort_unstable_by(|a, b| b.0.cmp(&a.0));

    let mut best_size = 0;
    let mut best_group = vec![];
    let candidates = vec![true; n];

    // iterate over nodes starting from the most connected to least connected for optimization since its faster
    for (_, &start) in degrees.iter() {
        // start with the first computer in the iter
        let mut group = vec![start];
        // start of with each computer being a potential candidate
        let mut potential = candidates.clone();
        // for all the direct connections of the starting node
        for node in graph[start].iter() {
            let index_current = node_to_idx[node];
            // if the current node is a potential candidate
            if potential[index_current] {
                // add it to the group
                group.push(node);

                // Update the best group if the current group is larger
                if group.len() > best_size {
                    best_size = group.len();
                    best_group = group.clone();
                    // exit since this group will never be connected to anything more than the current group by immediate connections
                    if best_size >= degrees[0].0 + 1 {
                        break;
                    }
                }
                // update all the potential candidates
                for i in 0..n {
                    // if it is marked true and a non immediate neighbour of the current node just skip it
                    if potential[i] && !adjacency_matrix[index_current][i] {
                        potential[i] = false;
                    }
                }
            }
        }
        // Early exit if the current group size cannot be improved
        if best_size >= degrees[0].0 + 1 {
            break;
        }
    }

    // Sort the best group alphabetically and return it
    best_group.sort();
    best_group.into_iter().cloned().collect()
}

fn part2(input: &str) -> String {
    let graph = build_graph(input);
    let party = find_lan_party(&graph);
    party.join(",")
}

#[test]
fn test_part1() {
    assert_eq!(part1(include_str!("../data/test/23.txt")), 7);
}

#[test]
fn test_part2() {
    assert_eq!(
        part2(include_str!("../data/test/23.txt")),
        "co,de,ka,ta".to_string()
    );
}
