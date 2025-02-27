use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq)]
pub struct Node {
    pub id: usize,
    pub name: String,
    pub links: Vec<Link>,
}

impl Node {
    pub fn new(name: &str, id: usize) -> Self {
        Self {
            id,
            name: name.into(),
            links: Vec::new(),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Link {
    pub to: usize,
    pub cost: u32,
}

impl Link {
    pub fn new(to: usize, cost: u32) -> Self {
        Self { to, cost }
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
    pub registered: bool,
}

impl Path {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            cost: 0,
            paths: Vec::new(),
            registered: false,
        }
    }
}

fn remove_first<T>(vec: &mut Vec<T>) -> Option<T> {
    if vec.is_empty() {
        return None;
    }
    Some(vec.remove(0))
}

pub fn spf(graph: &HashMap<usize, Node>, root: usize) -> HashMap<usize, Path> {
    let mut spf = HashMap::<usize, Path>::new();
    let mut paths = HashMap::<usize, Path>::new();
    let mut vq = Vec::<Path>::new();

    let mut c = Path::new(root);
    c.paths.push(vec![root]);

    paths.insert(root, c.clone());
    vq.push(c);

    while let Some(v) = remove_first(&mut vq) {
        spf.insert(v.id, v.clone());

        let Some(edge) = graph.get(&v.id) else {
            continue;
        };

        for link in edge.links.iter() {
            let c = paths.entry(link.to).or_insert_with(|| Path::new(link.to));

            if c.id == root {
                continue;
            }

            if c.cost != 0 && c.cost < v.cost + link.cost {
                continue;
            }

            if c.cost != 0 && c.cost == v.cost + link.cost {
                // Fall through.
            }

            if c.cost == 0 || c.cost > v.cost + link.cost {
                c.cost = v.cost.saturating_add(link.cost);
                c.paths.clear();
            }

            if v.id == root {
                let path = vec![root, c.id];
                c.paths.push(path);
            } else {
                for path in v.paths.iter() {
                    let mut newpath = path.clone();
                    newpath.push(c.id);
                    c.paths.push(newpath);
                }
            }

            if !c.registered {
                c.registered = true;
            } else {
                if let Some(rem_index) = vq.iter().position(|x| x.id == c.id) {
                    vq.remove(rem_index);
                }
            }
            vq.push(c.clone());
            vq.sort_by(|a, b| a.cost.cmp(&b.cost));
        }
    }
    spf
}

fn main() {
    let mut graph = HashMap::new();
    let mut nodes = vec![
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
        nodes[from].links.push(Link::new(to, cost));
    }

    for node in nodes {
        graph.insert(node.id, node);
    }

    let spf = spf(&graph, 0);

    for (node, path) in &spf {
        println!("node: {}", node);
        for p in &path.paths {
            println!("  metric {} path {:?}", path.cost, p);
        }
    }
}
