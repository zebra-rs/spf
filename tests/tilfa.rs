use std::collections::BTreeMap;

use spf::*;

pub fn tilfa_graph() -> BTreeMap<usize, Node> {
    let mut graph = BTreeMap::new();

    // Insert nodes
    let nodes = vec![
        Node::new("S", 0),
        Node::new("N1", 1),
        Node::new("N2", 2),
        Node::new("N3", 3),
        Node::new("R1", 4),
        Node::new("R2", 5),
        Node::new("R3", 6),
        Node::new("D", 7),
    ];

    for node in nodes {
        graph.insert(node.id, node);
    }

    // Define links
    let links = vec![
        // S
        (0, 1, 1),    // N1
        (0, 2, 1),    // N2
        (0, 3, 1000), // N3
        // N1
        (1, 0, 1), // S
        (1, 4, 1), // R1
        (1, 5, 1), // R2
        (1, 7, 1), // D
        // N2
        (2, 0, 1), // S
        (2, 4, 1), // R1
        // N3
        (3, 0, 1000), // S
        (3, 4, 1000), // R1
        // R1
        (4, 1, 1),    // N1
        (4, 2, 1),    // N2
        (4, 3, 1000), // N3
        (4, 5, 1000), // R2
        // R2
        (5, 1, 1),    // N1
        (5, 4, 1000), // R1
        (5, 6, 1000), // R3
        // R3
        (6, 5, 1000), // R2
        (6, 7, 1),    // D
        // D
        (7, 1, 1), // N1
        (7, 6, 1), // R3
    ];

    // Insert links into nodes
    for (from, to, cost) in links {
        graph
            .get_mut(&from)
            .unwrap()
            .olinks
            .push(Link::new(from, to, cost));
        graph
            .get_mut(&to)
            .unwrap()
            .ilinks
            .push(Link::new(from, to, cost));
    }

    graph
}

#[test]
pub fn tilfa1() {
    let graph = tilfa_graph();
    let s = 0;
    let d = 7;
    let x = 1;

    tilfa(&graph, s, d, x);
}

pub fn tilfa_graph_adj_seg() -> BTreeMap<usize, Node> {
    let mut graph = BTreeMap::new();

    // Insert nodes explicitly
    let nodes = vec![
        Node::new("S", 0),
        Node::new("R2", 1),
        Node::new("R3", 2),
        Node::new("R4", 3),
        Node::new("R5", 4),
        Node::new("R7", 5),
        Node::new("R8", 6),
        Node::new("R9", 7),
        Node::new("R10", 8),
        Node::new("D", 9),
    ];

    for node in nodes {
        graph.insert(node.id, node);
    }

    // Define and add links
    let links = vec![
        // S
        (0, 1, 1), // R2
        // R2
        (1, 0, 1),    // S
        (1, 2, 1),    // R3
        (1, 5, 1000), // R7
        // R3
        (2, 1, 1), // R2
        (2, 3, 1), // R4
        (2, 5, 1), // R7
        (2, 6, 1), // R8
        // R4
        (3, 2, 1),    // R3
        (3, 4, 1),    // R5
        (3, 6, 1000), // R8
        // R5
        (4, 3, 1), // R4
        (4, 9, 1), // D
        // R7
        (5, 1, 1000), // R2
        (5, 2, 1),    // R3
        (5, 6, 1000), // R8
        (5, 7, 1000), // R9
        // R8
        (6, 2, 1),    // R3
        (6, 3, 1000), // R4
        (6, 5, 1000), // R7
        (6, 8, 1),    // R10
        // R9
        (7, 5, 1000), // R7
        (7, 8, 1),    // R10
        // R10
        (8, 6, 1), // R8
        (8, 7, 1), // R9
        // D
        (9, 4, 1), // R5
    ];

    // Insert links into nodes
    for (from, to, cost) in links {
        graph
            .get_mut(&from)
            .unwrap()
            .olinks
            .push(Link::new(from, to, cost));
        graph
            .get_mut(&to)
            .unwrap()
            .ilinks
            .push(Link::new(from, to, cost));
    }

    graph
}

#[test]
pub fn tilfa2() {
    let graph = tilfa_graph_adj_seg();
    let s = 0;
    let d = 9; // D
    let x = 2; // R3

    tilfa(&graph, s, d, x);
}
