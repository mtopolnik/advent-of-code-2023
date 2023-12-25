use std::{
    cmp::Ordering::{Equal, Greater, Less},
    collections::{BTreeSet, HashMap, HashSet, VecDeque},
    fs::read_to_string,
};

type Node = [u8; 3];
type Edge = (Node, Node);

fn edge(src: Node, dst: Node) -> Edge {
    match src.cmp(&dst) {
        Less => (src, dst),
        Greater => (dst, src),
        Equal => panic!("Node going to itself"),
    }
}

fn main() {
    let input = read_to_string("input/day25.txt").unwrap();
    let mut nodes = BTreeSet::<Node>::new();
    let mut edges = BTreeSet::<Edge>::new();
    let mut graph = HashMap::<Node, BTreeSet<Edge>>::new();
    for line in input.lines() {
        let (src_str, dests_str) = line.split_once(": ").unwrap();
        let [s, r, c, ..] = *src_str.as_bytes() else {
            panic!("Parse error");
        };
        let src = [s, r, c];
        let dests: Vec<Node> = dests_str
            .split(" ")
            .map(|dst_str| {
                let [d, s, t, ..] = *dst_str.as_bytes() else {
                    panic!("Parse error");
                };
                [d, s, t]
            })
            .collect();
        nodes.insert(src);
        nodes.extend(dests.iter());
        for edge in dests.into_iter().map(|dst| edge(src, dst)) {
            let (n1, n2) = edge;
            edges.insert(edge);
            graph.entry(n1).or_default().insert(edge);
            graph.entry(n2).or_default().insert(edge);
        }
    }
    let mxv_size = group_size(&graph, node_from_string("mxv"));
    println!("mxv's group size: {}", mxv_size);
    let sdv_size = group_size(&graph, node_from_string("sdv"));
    println!("sdv's group size: {}", sdv_size);
    println!("Part 1: {}", mxv_size * sdv_size);
}

fn group_size(graph: &HashMap<Node, BTreeSet<Edge>>, start_node: Node) -> usize {
    let mut visited_nodes = HashSet::<Node>::new();
    let mut todo = VecDeque::<Node>::new();
    todo.push_back(start_node);
    loop {
        let Some(curr_node) = todo.pop_front() else {
            return visited_nodes.len();
        };
        visited_nodes.insert(curr_node);
        for edge in graph.get(&curr_node).unwrap() {
            let (n1, n2) = *edge;
            let other_node = if n1 != curr_node { n1 } else { n2 };
            if !visited_nodes.contains(&other_node) {
                todo.push_back(other_node);
            }
        }
    }
}

fn node_from_string(s: &str) -> Node {
    let [a, b, c] = s.as_bytes()[0..3] else {
        panic!("{s}")
    };
    [a, b, c]
}

fn node_to_string(n: Node) -> String {
    String::from_utf8_lossy(&n).to_string()
}

fn edge_to_string(e: Edge) -> String {
    let (n1, n2) = e;
    format!(
        "{}/{}",
        String::from_utf8_lossy(&n1),
        String::from_utf8_lossy(&n2)
    )
}
