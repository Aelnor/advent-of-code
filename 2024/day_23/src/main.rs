use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_input() -> HashMap<String, HashSet<String>> {
    let reader = BufReader::new(File::open("data").unwrap());
    let mut connections = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();

        let (left, right) = line.split_once('-').unwrap();
        connections.push((left.to_string(), right.to_string()));
    }

    let mut graph = HashMap::new();

    for (c1, c2) in connections {
        graph
            .entry(c1.clone())
            .or_insert(HashSet::new())
            .insert(c2.clone());
        graph
            .entry(c2.clone())
            .or_insert(HashSet::new())
            .insert(c1.clone());
    }
    graph
}

fn part1(graph: &HashMap<String, HashSet<String>>) -> usize {
    let mut seqs = HashSet::new();

    for k in graph.keys() {
        for v in &graph[k] {
            for v3 in &graph[v] {
                if graph[k].contains(v3) {
                    let mut vec = vec![k, v, v3];
                    vec.sort();
                    seqs.insert(vec);
                }
            }
        }
    }

    seqs.into_iter()
        .filter(|v| {
            v[0].chars().next().unwrap() == 't'
                || v[1].chars().next().unwrap() == 't'
                || v[2].chars().next().unwrap() == 't'
        })
        .count()
}

fn part2(graph: &HashMap<String, HashSet<String>>) -> String {
    let r = HashSet::new();
    let p: HashSet<_> = graph.keys().cloned().collect();
    let x = HashSet::new();
    let mut cliques = Vec::new();
    bron_kerbosch_with_pivot(r, p, x, &graph, &mut cliques);

    let mut longest_set: Vec<_> = cliques
        .into_iter()
        .max_by_key(|set| set.len())
        .unwrap()
        .into_iter()
        .collect();

    longest_set.sort();

    longest_set.join(",")
}

fn bron_kerbosch_with_pivot(
    r: HashSet<String>,
    p: HashSet<String>,
    x: HashSet<String>,
    graph: &HashMap<String, HashSet<String>>,
    cliques: &mut Vec<HashSet<String>>,
) {
    if p.is_empty() && x.is_empty() {
        cliques.push(r);
        return;
    }
    // Choose a pivot vertex from P ∪ X
    let pivot = p.union(&x).next().unwrap();

    // Iterate over vertices in P that are not neighbors of the pivot
    for v in p.difference(&graph[pivot]) {
        let mut r_new = r.clone();
        r_new.insert(v.to_string());

        let p_new: HashSet<_> = p.intersection(&graph[v]).cloned().collect();
        let x_new: HashSet<_> = x.intersection(&graph[v]).cloned().collect();

        bron_kerbosch_with_pivot(r_new, p_new, x_new, graph, cliques);

        // Move v from P to X
        let mut p_mut = p.clone();
        let mut x_mut = x.clone();
        p_mut.remove(v);
        x_mut.insert(v.to_string());
    }
}

fn main() {
    let graph = parse_input();
    println!("part 1: {}", part1(&graph));
    println!("part 2: {}", part2(&graph));
}
