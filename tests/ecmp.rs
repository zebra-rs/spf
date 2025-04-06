use std::collections::BTreeMap;
use std::time;

use spf::*;

pub fn ecmp_topology() -> Graph {
    let mut graph = BTreeMap::new();

    // First, insert all nodes
    let nodes = vec![
        Node::new("N1", 0),
        Node::new("N2", 1),
        Node::new("N3", 2),
        Node::new("N4", 3),
        Node::new("N5", 4),
    ];

    // Insert each Node into the map keyed by node id
    for node in nodes {
        graph.insert(node.id, node);
    }

    // Define links between nodes
    let links = vec![
        (0, 1, 10),
        (0, 2, 10),
        (1, 0, 10),
        (1, 2, 5),
        (1, 3, 10),
        (2, 0, 10),
        (2, 1, 5),
        (2, 3, 10),
        (3, 1, 10),
        (3, 2, 10),
        (3, 4, 10),
        (4, 3, 10),
    ];

    // Now add links to the respective nodes stored in our BTreeMap
    for (from, to, cost) in links {
        graph
            .get_mut(&from)
            .unwrap()
            .olinks
            .push(Link::new(from, to, cost));
    }

    graph
}
#[test]
pub fn ecmp() {
    let opt = SpfOpt {
        full_path: true,
        path_max: 32,
        srv6: false,
        srmpls: true,
    };

    let graph = ecmp_topology();

    let now = time::Instant::now();
    let spf = spf(&graph, 0, opt.full_path, opt.path_max);
    println!("Time ecmp {:?}", now.elapsed());

    // node: 0 nexthops: 1
    //   metric 0 path [0]
    // node: 1 nexthops: 1
    //   metric 10 path [0, 1]
    // node: 2 nexthops: 1
    //   metric 10 path [0, 2]
    // node: 3 nexthops: 2
    //   metric 20 path [0, 1, 3]
    //   metric 20 path [0, 2, 3]
    // node: 4 nexthops: 2
    //   metric 30 path [0, 1, 3, 4]
    //   metric 30 path [0, 2, 3, 4]

    assert_eq!(spf.len(), 5);

    let node4 = spf.get(&4).unwrap();
    assert_eq!(node4.paths.len(), 2);

    disp(&spf, opt.full_path)
}
