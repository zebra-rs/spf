use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::rc::Rc;

#[derive(Debug)]
pub struct Graph {
    pub graph: HashMap<usize, Node>,
}

type RefPath = Rc<RefCell<Path>>;

impl Graph {
    pub fn new() -> Self {
        Self {
            graph: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, node: Node) {
        self.graph.insert(node.id, node);
    }

    pub fn spf(&self, root: usize) -> HashMap<usize, RefPath> {
        let mut spf = HashMap::new();
        let mut paths = HashMap::<usize, RefPath>::new();
        let mut pq = BinaryHeap::<RefPath>::new();

        let c = Rc::new(RefCell::new(Path::new(root)));
        c.borrow_mut().paths.push(vec![root]);

        paths.insert(root, c.clone());
        pq.push(c.clone());

        while let Some(c) = pq.pop() {
            let v = c.clone();

            spf.insert(v.borrow().node, v.clone());

            let Some(node) = self.graph.get(&c.borrow().node) else {
                continue;
            };

            for link in node.links.iter() {
                let c = paths
                    .entry(link.to)
                    .or_insert_with(|| Rc::new(RefCell::new(Path::new(link.to))))
                    .clone();

                if c.borrow().node == root {
                    continue;
                }

                let c_cost = c.borrow().cost;
                let v_cost = v.borrow().cost + link.cost;

                if c_cost != 0 && c_cost < v_cost {
                    continue;
                }

                if c_cost == 0 || c_cost > v_cost {
                    let mut c_mut = c.borrow_mut();
                    c_mut.cost = v_cost;
                    c_mut.paths.clear();
                }

                let updated_paths: Vec<_> = if v.borrow().node == root {
                    vec![vec![root, c.borrow().node]]
                } else {
                    v.borrow()
                        .paths
                        .iter()
                        .map(|p| {
                            let mut path = p.clone();
                            path.push(c.borrow().node);
                            path
                        })
                        .collect()
                };

                c.borrow_mut().paths.extend(updated_paths);

                if !c.borrow().registered {
                    pq.push(c.clone());
                    c.borrow_mut().registered = true;
                } else {
                    // TODO: We need to re-calculate pq with potentially updated c.cost.
                }
            }
        }
        spf
    }
}

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
    pub from: usize,
    pub to: usize,
    pub cost: u32,
}

impl Link {
    pub fn new(from: usize, to: usize, cost: u32) -> Self {
        Self { from, to, cost }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)] // Added Clone for easier conversion
pub struct Path {
    pub node: usize,
    pub cost: u32,
    pub paths: Vec<Vec<usize>>,
    pub registered: bool,
}

impl Path {
    pub fn new(node: usize) -> Self {
        Self {
            node,
            cost: 0,
            paths: Vec::new(),
            registered: false,
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

fn main() {
    let mut graph = Graph::new();
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
        nodes[from].links.push(Link::new(from, to, cost));
    }

    for node in nodes {
        graph.add_node(node);
    }

    let spf = graph.spf(0);

    for (node, path) in &spf {
        let path = path.borrow();
        for p in &path.paths {
            println!("{} metric {} path: {:?}", node, path.cost, p);
        }
    }
}
