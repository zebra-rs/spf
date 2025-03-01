use std::cmp::Ordering;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::time;

type Graph = Vec<Node>;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Node {
    pub id: usize,
    pub name: String,
    pub olinks: Vec<Link>,
    pub ilinks: Vec<Link>,
    pub is_disabled: bool,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum SpfDirect {
    Normal,
    Reverse,
}

impl Node {
    pub fn new(name: &str, id: usize) -> Self {
        Self {
            id,
            name: name.into(),
            olinks: Vec::new(),
            ilinks: Vec::new(),
            is_disabled: false,
        }
    }

    pub fn links(&self, direct: &SpfDirect) -> &Vec<Link> {
        if *direct == SpfDirect::Normal {
            &self.olinks
        } else {
            &self.ilinks
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Link {
    pub from: usize,
    pub to: usize,
    pub cost: u32,
}

impl Link {
    pub fn new(from: usize, to: usize, cost: u32) -> Self {
        Self { from, to, cost }
    }

    pub fn id(&self, direct: &SpfDirect) -> usize {
        if *direct == SpfDirect::Normal {
            self.to
        } else {
            self.from
        }
    }
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

#[derive(Debug, Eq, PartialEq, Clone)] // Added Clone for easier conversion
pub struct Path {
    pub id: usize,
    pub cost: u32,
    pub paths: Vec<Vec<usize>>,
    pub nexthops: HashSet<Vec<usize>>,
    pub registered: bool,
}

impl Path {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            cost: 0,
            paths: Vec::new(),
            nexthops: HashSet::new(),
            registered: false,
        }
    }
}

pub fn spf(
    graph: &Vec<Node>,
    root: usize,
    full_path: bool,
    path_max: usize,
    direct: &SpfDirect,
) -> BTreeMap<usize, Path> {
    let mut spf = BTreeMap::<usize, Path>::new();
    let mut paths = HashMap::<usize, Path>::new();
    let mut bt = BTreeMap::<(u32, usize), Path>::new();

    let mut c = Path::new(root);
    c.paths.push(vec![root]);
    c.nexthops.insert(vec![root]);

    paths.insert(root, c.clone());
    bt.insert((c.cost, root), c);

    while let Some((_, v)) = bt.pop_first() {
        spf.insert(v.id, v.clone());

        let Some(edge) = graph.get(v.id) else {
            continue;
        };

        if edge.is_disabled {
            continue;
        }

        for link in edge.links(direct).iter() {
            if let Some(x) = graph.get(link.id(direct)) {
                if x.is_disabled {
                    continue;
                }
            };

            let c = paths
                .entry(link.id(direct))
                .or_insert_with(|| Path::new(link.id(direct)));

            let ocost = c.cost;

            if c.id == root {
                continue;
            }

            if c.cost != 0 && c.cost < v.cost + link.cost {
                continue;
            }

            if c.cost != 0 && c.cost == v.cost + link.cost {
                // Fall through for ECMP.
            }

            if c.cost == 0 || c.cost > v.cost + link.cost {
                c.cost = v.cost.saturating_add(link.cost);
                c.paths.clear();
            }

            if v.id == root {
                let path = vec![root, c.id];

                if full_path {
                    c.paths.push(path);
                } else {
                    c.nexthops.insert(path);
                }
            } else {
                if full_path {
                    for path in &v.paths {
                        if path_max == 0 || c.paths.len() < path_max {
                            let mut newpath = path.clone();
                            newpath.push(c.id);
                            c.paths.push(newpath);
                        }
                    }
                } else {
                    for nhop in &v.nexthops {
                        if path_max == 0 || c.nexthops.len() < path_max {
                            let mut newnhop = nhop.clone();
                            if nhop.len() < 2 {
                                newnhop.push(c.id);
                            }
                            c.nexthops.insert(newnhop);
                        }
                    }
                }
            }

            if !c.registered {
                c.registered = true;
                bt.insert((c.cost, c.id), c.clone());
            } else {
                if ocost == c.cost {
                    if let Some(v) = bt.get_mut(&(c.cost, c.id)) {
                        if full_path {
                            v.paths = c.paths.clone();
                        } else {
                            v.nexthops = c.nexthops.clone();
                        }
                    }
                } else {
                    bt.remove(&(ocost, c.id));
                    bt.insert((c.cost, c.id), c.clone());
                }
            }
        }
    }
    spf
}

pub fn spf_normal(
    graph: &Vec<Node>,
    root: usize,
    full_path: bool,
    path_max: usize,
) -> BTreeMap<usize, Path> {
    spf(graph, root, full_path, path_max, &SpfDirect::Normal)
}

pub fn spf_reverse(
    graph: &Vec<Node>,
    root: usize,
    full_path: bool,
    path_max: usize,
) -> BTreeMap<usize, Path> {
    spf(graph, root, full_path, path_max, &SpfDirect::Reverse)
}

pub fn p_space_nodes(graph: &Vec<Node>, s: usize, x: usize) -> Vec<usize> {
    let mut nodes = Vec::<usize>::new();

    let spf = spf_normal(graph, s, true, 0);

    for (node, path) in &spf {
        if *node == s {
            continue;
        }
        let mut found_x = false;
        for path in &path.paths {
            for p in path.iter() {
                if *p == x {
                    found_x = true;
                }
            }
        }
        if !found_x {
            nodes.push(*node);
        }
    }

    nodes
}

pub fn q_space_nodes(graph: &Vec<Node>, d: usize, x: usize) -> Vec<usize> {
    let mut nodes = Vec::<usize>::new();

    let spf = spf_reverse(graph, d, true, 0);

    for (node, path) in spf.iter() {
        if *node == d {
            continue;
        }
        let mut found_x = false;
        for path in &path.paths {
            for p in path.iter() {
                if *p == x {
                    found_x = true;
                }
            }
        }
        if !found_x {
            nodes.push(*node);
        }
    }

    nodes
}

//  +---+ +---+ +---+ +---+
//  | 0 |-| 1 |-|...|-|n-1|
//  +---+ +---+ +---+ +---+
//    |     |     |     |
//  +---+ +---+ +---+ +---+
//  | n |-|n+1|-|...|-|2n|
//  +---+ +---+ +---+ +---+
//    |     |     |     |
//  +---+ +---+ +---+ +---+
//  |Xn |-| 1 |-|...|-|   |
//  +---+ +---+ +---+ +---+
//
pub fn bench(n: usize, opt: &SpfOpt) {
    let mut graph = vec![];
    for i in 0..n {
        for j in 0..n {
            let id = (i * n) + j;
            let mut node = Node::new(&id.to_string(), id);
            // Vertical link: do not create vertical link for bottom row.
            if i != n - 1 {
                // Link from id to id + n.
                let link = Link::new(id, id + n, 10);
                node.olinks.push(link);
            }
            // Horisontal link: do not create horisontal link for most right column.
            if j != n - 1 {
                // Link from id to id + n.
                let link = Link::new(id, id + 1, 10);
                node.olinks.push(link);
            }
            graph.push(node);
        }
    }

    let now = time::Instant::now();
    let _spf = spf_normal(&graph, 0, opt.full_path, opt.path_max);
    println!("n:{} {:?}", n, now.elapsed());

    // disp(&spf, opt.full_path)
}

pub fn ecmp_topology() -> Graph {
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

pub fn ecmp(opt: &SpfOpt) {
    let graph = ecmp_topology();

    let now = time::Instant::now();
    let spf = spf_normal(&graph, 0, opt.full_path, opt.path_max);
    println!("ecmp {:?}", now.elapsed());

    disp(&spf, opt.full_path)
}

pub fn pc_path(graph: &Vec<Node>, d: usize, x: usize) -> Vec<usize> {
    let mut pc_graph = graph.clone();
    let node = pc_graph.get_mut(x).unwrap();
    node.is_disabled = true;
    let mut pc_spf = spf_normal(&pc_graph, 0, true, 0);

    let mut pc_path = pc_spf.remove(&d).unwrap();
    pc_path.paths.remove(0)
}

#[derive(Default)]
pub struct SpfOpt {
    pub full_path: bool,
    pub path_max: usize,
}

impl SpfOpt {
    pub fn new() -> Self {
        Self::default()
    }
}

pub fn tilfa_graph() -> Vec<Node> {
    let mut graph = vec![
        Node::new("S", 0),
        Node::new("N1", 1),
        Node::new("N2", 2),
        Node::new("N3", 3),
        Node::new("R1", 4),
        Node::new("R2", 5),
        Node::new("R3", 6),
        Node::new("D", 7),
    ];

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
        (6, 5, 1),    // R2
        (6, 7, 1000), // D
        // D
        (7, 1, 1), // N1
        (7, 6, 1), // R3
    ];

    for (from, to, cost) in links {
        graph[from].olinks.push(Link::new(from, to, cost));
        graph[to].ilinks.push(Link::new(from, to, cost));
    }
    graph
}

pub fn tilfa(opt: &SpfOpt) {
    let graph = tilfa_graph();
    let s = 0;
    let d = 7;
    let x = 1;

    // Normal SPF
    // let spt = spf_normal(&graph, 0, opt.full_path, opt.path_max);

    let p_nodes = p_space_nodes(&graph, s, x);
    let q_nodes = q_space_nodes(&graph, d, x);
    // let pc_path = pc_path(&graph, s, d);

    // P
    print!("P:");
    for name in p_nodes
        .iter()
        .filter_map(|p| graph.get(*p).map(|n| &n.name))
    {
        print!(" {}", name);
    }
    println!();
    // Q
    println!("Q: {:?}", q_nodes);
    // println!("PCPath: {:?}", pc_path);
}

pub fn disp(spf: &BTreeMap<usize, Path>, full_path: bool) {
    if full_path {
        for (node, path) in spf {
            println!("node: {} nexthops: {}", node, path.paths.len());
            for p in &path.paths {
                println!("  metric {} path {:?}", path.cost, p);
            }
        }
    } else {
        for (node, nhops) in spf {
            println!("node: {} nexthops: {}", node, nhops.nexthops.len());
            for p in &nhops.nexthops {
                println!("  metric {} path {:?}", nhops.cost, p);
            }
        }
    }
}

pub fn intersect(sa: &Vec<usize>, sb: &Vec<usize>, sc: &Vec<usize>) -> Vec<usize> {
    let mut result = Vec::new();

    for na in sa {
        for nb in sb {
            for nc in sc {
                if na == nb && nb == nc {
                    if !result.iter().any(|x: &usize| x == na) {
                        result.push(*na);
                    }
                }
            }
        }
    }

    result
}

pub fn intersect_test() {
    // Example test case
    let sa = vec![1, 2, 3];
    let sb = vec![2, 3, 4];
    let sc = vec![3, 4, 5];

    let intersection = intersect(&sa, &sb, &sc);
    println!("{:?}", intersection);
}

fn main() {
    let opt = SpfOpt {
        full_path: true,
        path_max: 16,
    };
    // ecmp(&opt);
    // bench(300, &opt);
    tilfa(&opt);
}
