use std::cmp::Ordering;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::time;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Node {
    pub id: usize,
    pub name: String,
    pub olinks: Vec<Link>,
    pub ilinks: Vec<Link>,
    pub is_disabled: bool,
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

pub fn spf(graph: &Vec<Node>, root: usize, full_path: bool) -> BTreeMap<usize, Path> {
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

        for link in edge.olinks.iter() {
            if let Some(x) = graph.get(link.to) {
                if x.is_disabled {
                    continue;
                }
            };

            let c = paths.entry(link.to).or_insert_with(|| Path::new(link.to));
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
                if full_path {
                    let path = vec![root, c.id];
                    c.paths.push(path);
                } else {
                    let nhop = vec![root, c.id];
                    c.nexthops.insert(nhop);
                }
            } else {
                if full_path {
                    for path in v.paths.iter() {
                        let mut newpath = path.clone();
                        newpath.push(c.id);
                        c.paths.push(newpath);
                    }
                } else {
                    for nhop in v.nexthops.iter() {
                        let mut newnhop = nhop.clone();
                        if nhop.len() < 2 {
                            newnhop.push(c.id);
                        }
                        c.nexthops.insert(newnhop);
                    }
                }
            }

            if !c.registered {
                c.registered = true;
                bt.insert((c.cost, c.id), c.clone());
            } else {
                if ocost == c.cost {
                    if let Some(v) = bt.get_mut(&(c.cost, c.id)) {
                        v.paths = c.paths.clone();
                        v.nexthops = c.nexthops.clone();
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

pub fn spf_reverse(graph: &Vec<Node>, dest: usize, full_path: bool) -> BTreeMap<usize, Path> {
    let mut spf = BTreeMap::<usize, Path>::new();
    let mut paths = HashMap::<usize, Path>::new();
    let mut bt = BTreeMap::<(u32, usize), Path>::new();

    let mut c = Path::new(dest);
    c.paths.push(vec![dest]);
    c.nexthops.insert(vec![dest]);

    paths.insert(dest, c.clone());
    bt.insert((c.cost, dest), c);

    while let Some((_, v)) = bt.pop_first() {
        spf.insert(v.id, v.clone());

        let Some(edge) = graph.get(v.id) else {
            continue;
        };

        for link in edge.ilinks.iter() {
            let c = paths
                .entry(link.from)
                .or_insert_with(|| Path::new(link.from));
            let ocost = c.cost;

            if c.id == dest {
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

            if v.id == dest {
                if full_path {
                    let path = vec![dest, c.id];
                    c.paths.push(path);
                } else {
                    let nhop = vec![dest, c.id];
                    c.nexthops.insert(nhop);
                }
            } else {
                if full_path {
                    for path in v.paths.iter() {
                        let mut newpath = path.clone();
                        newpath.push(c.id);
                        c.paths.push(newpath);
                    }
                } else {
                    for nhop in v.nexthops.iter() {
                        let mut newnhop = nhop.clone();
                        if nhop.len() < 2 {
                            newnhop.push(c.id);
                        }
                        c.nexthops.insert(newnhop);
                    }
                }
            }

            if !c.registered {
                c.registered = true;
                bt.insert((c.cost, c.id), c.clone());
            } else {
                if ocost == c.cost {
                    if let Some(v) = bt.get_mut(&(c.cost, c.id)) {
                        v.paths = c.paths.clone();
                        v.nexthops = c.nexthops.clone();
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

pub fn p_space_nodes(graph: &Vec<Node>, root: usize, x: usize) -> Vec<usize> {
    let mut nodes = Vec::<usize>::new();

    let spf = spf(graph, root, true);

    for (node, path) in spf.iter() {
        if *node == root {
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

    let spf = spf_reverse(graph, d, true);

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
pub fn bench(n: usize, full_path: bool, debug: bool) {
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
    let spf = spf(&graph, 0, full_path);
    println!("n:{} {:?}", n, now.elapsed());

    if debug {
        disp(&spf, full_path)
    }
}

pub fn ecmp(full_path: bool, debug: bool) {
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

    let now = time::Instant::now();
    let spf = spf(&graph, 0, full_path);
    println!("ecmp {:?}", now.elapsed());

    if debug {
        disp(&spf, full_path)
    }
}

pub fn pc_path(graph: &Vec<Node>, d: usize, x: usize) -> Option<Path> {
    let mut pc_graph = graph.clone();
    let node = pc_graph.get_mut(x).unwrap();
    node.is_disabled = true;
    let mut pc_spf = spf(&pc_graph, 0, true);
    pc_spf.remove(&d)
}

pub fn tilfa(_full_path: bool, _debug: bool) {
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
        (1, 3, 10),
        (2, 0, 10),
        (2, 4, 10),
        (3, 1, 10),
        (3, 4, 50),
        (4, 2, 10),
        (4, 3, 50),
    ];

    for (from, to, cost) in links {
        graph[from].olinks.push(Link::new(from, to, cost));
        graph[to].ilinks.push(Link::new(from, to, cost));
    }

    //let now = time::Instant::now();
    let p_nodes = p_space_nodes(&graph, 0, 2);
    let q_nodes = q_space_nodes(&graph, 2, 0);
    let pc_path = pc_path(&graph, 4, 2).unwrap();

    println!("P: {:?}", p_nodes);
    println!("Q: {:?}", q_nodes);
    println!("PCPath: {:?}", pc_path);
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

fn main() {
    let full_path = true;
    let debug = true;

    // bench(1000, full_path, debug);
    // ecmp(full_path, debug);
    tilfa(full_path, debug);
}
