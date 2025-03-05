use std::time;

use spf::*;

#[test]
fn ecmp_topology() -> Graph {
    let mut graph = vec![
        Node::new("N1", 0),
        Node::new("N2", 1),
        Node::new("N3", 2),
        Node::new("N4", 3),
        Node::new("N5", 4),
    ];

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

    for (from, to, cost) in links {
        graph[from].olinks.push(Link::new(from, to, cost));
    }
    graph
}

mfn spf_nexthop_only(opt: &SpfOpt) {
    let graph = ecmp_topology();

    let now = time::Instant::now();
    let spf = spf_normal(&graph, 0, opt.full_path, opt.path_max);
    println!("ecmp {:?}", now.elapsed());

    disp(&spf, opt.full_path)
}
