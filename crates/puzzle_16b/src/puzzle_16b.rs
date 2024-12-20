use std::cmp::Reverse;
use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    fs,
};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Space {
    Empty,
    Wall,
    Start,
    End,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Dir {
    North = 0,
    East = 1,
    South = 2,
    West = 3,
}

impl Dir {
    fn offset(&self) -> (isize, isize) {
        match self {
            Dir::North => (-1, 0),
            Dir::East => (0, 1),
            Dir::South => (1, 0),
            Dir::West => (0, -1),
        }
    }
    fn cost(self, other: Self) -> usize {
        match (self as u8).abs_diff(other as u8) {
            0 => 1,
            1 => 1001,
            2 => 2001,
            3 => 1001,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct NodeKey {
    x: usize,
    y: usize,
    dir: Dir,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct NodeVal {
    cost: usize,
    path: bool,
    prev: Vec<NodeKey>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Node {
    key: NodeKey,
    val: NodeVal,
}

impl Node {
    fn new(from: &Self, dir: Dir) -> Self {
        let offset = dir.offset();
        Self {
            key: NodeKey {
                x: from.key.x.checked_add_signed(offset.0).unwrap(),
                y: from.key.y.checked_add_signed(offset.1).unwrap(),
                dir,
            },
            val: NodeVal {
                cost: from.val.cost + from.key.dir.cost(dir),
                path: false,
                prev: vec![],
            },
        }
    }

    fn neighbors(&self) -> [Node; 4] {
        [
            Node::new(self, Dir::North),
            Node::new(self, Dir::East),
            Node::new(self, Dir::South),
            Node::new(self, Dir::West),
        ]
    }
}

impl PartialOrd for NodeVal {
    fn partial_cmp(&self, rhs: &Self) -> Option<std::cmp::Ordering> {
        self.cost.partial_cmp(&rhs.cost)
    }
}

impl Ord for NodeVal {
    fn cmp(&self, rhs: &Self) -> std::cmp::Ordering {
        self.cost.cmp(&rhs.cost)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, rhs: &Self) -> Option<std::cmp::Ordering> {
        self.val.partial_cmp(&rhs.val)
    }
}

impl Ord for Node {
    fn cmp(&self, rhs: &Self) -> std::cmp::Ordering {
        self.val.cmp(&rhs.val)
    }
}

fn parse_file(s: &str) -> Vec<Vec<Space>> {
    s.lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '.' => Space::Empty,
                    '#' => Space::Wall,
                    'S' => Space::Start,
                    'E' => Space::End,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect()
}

fn find_start(data: &Vec<Vec<Space>>) -> NodeKey {
    for (x, r) in data.iter().enumerate() {
        for (y, &s) in r.iter().enumerate() {
            if s == Space::Start {
                return NodeKey {
                    x,
                    y,
                    dir: Dir::East,
                };
            }
        }
    }
    unreachable!()
}

#[allow(unused)]
fn dump_map(data: &Vec<Vec<Space>>, path_nodes: &HashSet<(usize, usize)>) {
    let x: Vec<_> = data
        .iter()
        .enumerate()
        .map(|(i, r)| {
            r.iter()
                .enumerate()
                .map(|(j, s)| match s {
                    Space::Wall => '#',
                    _ => {
                        if path_nodes.contains(&(i, j)) {
                            'O'
                        } else {
                            '.'
                        }
                    }
                })
                .collect::<String>()
        })
        .collect();
    for l in x {
        println!("{l}");
    }
}

fn solve(data: &Vec<Vec<Space>>) -> (usize, usize) {
    // too lazy to handle all the adjustments for horizontal enumeration above and the problem space is small
    let start = find_start(data);
    let mut nodes = HashMap::new();
    let first = NodeVal {
        cost: 0,
        path: false,
        prev: vec![],
    };

    nodes.insert(start, first.clone());
    let mut candidates = BinaryHeap::new();
    candidates.push(Reverse(Node {
        key: start,
        val: first,
    }));

    while let Some(Reverse(curr)) = candidates.pop() {
        if data[curr.key.x][curr.key.y] == Space::End {
            let mut s = vec![curr.key];
            let mut path_nodes = HashSet::new();
            while let Some(k) = s.pop() {
                let n = nodes.get(&k).unwrap();
                path_nodes.insert((k.x, k.y));
                for &back in &n.prev {
                    s.push(back);
                }
            }
            // dump_map(data, &path_nodes);

            //         part A          part B
            return (curr.val.cost, path_nodes.len());
        }

        {
            // why bother with interior mutability when you could just do more unnecessary work?
            let n = nodes.get_mut(&curr.key).unwrap();
            if n.path {
                continue;
            }
            n.path = true;
        }

        for n in curr.neighbors() {
            if data[n.key.x][n.key.y] == Space::Wall {
                continue;
            }

            let mut insert = true;
            let val = nodes
                .entry(n.key)
                .and_modify(|v| {
                    if n.val < *v {
                        assert!(!v.path);
                        *v = n.val.clone();
                    } else {
                        insert = false;
                        if n.val.cost == v.cost {
                            v.prev.push(curr.key);
                        }
                    }
                })
                .or_insert(n.val);

            if insert {
                val.prev.push(curr.key);
                candidates.push(Reverse(Node {
                    key: n.key,
                    val: val.clone(),
                }));
            }
        }
    }
    unreachable!()
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let input = &args[1];
    let data = fs::read_to_string(input).unwrap();
    let v = parse_file(&data);
    println!("{:?}", solve(&v));
}