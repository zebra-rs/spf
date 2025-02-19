use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

#[derive(Debug, Eq, PartialEq)]
struct Link {
    from: usize,
    to: usize,
    cost: u32,
}

#[derive(Debug, Eq, PartialEq)]
struct Path {
    node: usize,
    cost: u32,
    paths: Vec<Vec<usize>>,
}

impl Ord for Path {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn spf_calc(start: usize, graph: &HashMap<usize, Vec<Link>>) -> HashMap<usize, Path> {
    let mut pq = BinaryHeap::new();
    let mut spf: HashMap<usize, Path> = HashMap::new();

    pq.push(Path {
        node: start,
        cost: 0,
        paths: vec![vec![]],
    });

    while let Some(Path { node, cost, paths }) = pq.pop() {
        if let Some(existing) = spf.get_mut(&node) {
            if existing.cost == cost {
                existing.paths.extend(paths);
            }
            continue;
        }

        spf.insert(
            node,
            Path {
                node,
                cost,
                paths: paths.clone(),
            },
        );

        if let Some(neighbors) = graph.get(&node) {
            for link in neighbors {
                let new_cost = cost + link.cost;
                let mut new_paths = vec![];
                for path in &paths {
                    let mut new_path = path.clone();
                    new_path.push(link.to);
                    new_paths.push(new_path);
                }
                pq.push(Path {
                    node: link.to,
                    cost: new_cost,
                    paths: new_paths,
                });
            }
        }
    }

    spf
}

fn main() {
    let mut graph: HashMap<usize, Vec<Link>> = HashMap::new();
    graph.insert(
        1,
        vec![
            Link {
                from: 1,
                to: 2,
                cost: 10,
            },
            Link {
                from: 1,
                to: 3,
                cost: 10,
            },
        ],
    );
    graph.insert(
        2,
        vec![
            Link {
                from: 2,
                to: 1,
                cost: 10,
            },
            Link {
                from: 2,
                to: 3,
                cost: 5,
            },
            Link {
                from: 2,
                to: 4,
                cost: 10,
            },
        ],
    );
    graph.insert(
        3,
        vec![
            Link {
                from: 3,
                to: 1,
                cost: 10,
            },
            Link {
                from: 3,
                to: 2,
                cost: 5,
            },
            Link {
                from: 3,
                to: 4,
                cost: 10,
            },
        ],
    );
    graph.insert(
        4,
        vec![
            Link {
                from: 4,
                to: 2,
                cost: 10,
            },
            Link {
                from: 4,
                to: 3,
                cost: 10,
            },
            Link {
                from: 4,
                to: 5,
                cost: 10,
            },
        ],
    );
    graph.insert(
        5,
        vec![Link {
            from: 5,
            to: 4,
            cost: 10,
        }],
    );

    let result = spf_calc(1, &graph);

    for (node, path) in &result {
        println!("Node: {} Cost: {} Paths: {:?}", node, path.cost, path.paths);
    }
}
