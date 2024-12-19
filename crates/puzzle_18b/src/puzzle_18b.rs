use nom::{
    character::complete::{self, char, line_ending},
    combinator::{all_consuming, eof, map},
    multi::{many0_count, many1},
    sequence::{preceded, separated_pair, terminated},
    IResult,
};
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    fs,
};
use rayon::prelude::*;

const DIM: usize = 71;
type RowData = (usize, usize);

fn parse_line(s: &str) -> IResult<&str, RowData> {
    map(
        separated_pair(complete::u64, char(','), complete::u64),
        |(a, b)| (1 + b as usize, 1 + a as usize),
    )(s)
}

fn parse_file(s: &str) -> IResult<&str, Vec<RowData>> {
    let (s, v) = many1(terminated(parse_line, line_ending))(s)?;
    let (s, _) = all_consuming(preceded(many0_count(line_ending), eof))(s)?;
    Ok((s, v))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct NodeKey {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct NodeVal {
    cost: usize,
    path: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Node {
    key: NodeKey,
    val: NodeVal,
}

impl Node {
    fn new(from: &Self, offset: (isize, isize)) -> Self {
        Self {
            key: NodeKey {
                x: from.key.x.checked_add_signed(offset.0).unwrap(),
                y: from.key.y.checked_add_signed(offset.1).unwrap(),
            },
            val: NodeVal {
                cost: from.val.cost + 1,
                path: false,
            },
        }
    }

    fn neighbors(&self) -> [Node; 4] {
        [
            Node::new(self, (-1, 0)),
            Node::new(self, (0, 1)),
            Node::new(self, (1, 0)),
            Node::new(self, (0, -1)),
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

#[allow(unused)]
fn print_board(board: &Vec<Vec<bool>>, nodes: &HashMap<NodeKey, NodeVal>) {
    board
        .iter()
        .enumerate()
        .map(|(x, r)| {
            r.iter()
                .enumerate()
                .map(|(y, &b)| {
                    if b {
                        '#'
                    } else if nodes
                        .get(&NodeKey { x, y })
                        .map(|v| v.path)
                        .unwrap_or(false)
                    {
                        'o'
                    } else {
                        '.'
                    }
                })
                .collect::<String>()
        })
        .for_each(|s| println!("{s}"));
}

fn solve(data: &[RowData], n: usize) -> bool {
    let mut board = vec![vec![false; DIM]; DIM];
    for r in &mut board {
        r.insert(0, true);
        r.push(true);
    }
    board.insert(0, vec![true; DIM + 2]);
    board.push(vec![true; DIM + 2]);

    for &(x, y) in &data[0..n] {
        board[x][y] = true;
    }

    let start = NodeKey { x: 1, y: 1 };
    let first = NodeVal {
        cost: 0,
        path: false,
    };

    let mut nodes = HashMap::new();
    nodes.insert(start, first);
    let mut candidates = BinaryHeap::new();
    candidates.push(Reverse(Node {
        key: start,
        val: first,
    }));

    while let Some(Reverse(curr)) = candidates.pop() {
        if curr.key.x == DIM && curr.key.y == DIM {
            return false;
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
            if board[n.key.x][n.key.y] {
                continue;
            }

            let mut insert = true;
            let val = nodes
                .entry(n.key)
                .and_modify(|v| {
                    if n.val < *v {
                        assert!(!v.path);
                        *v = n.val;
                    } else {
                        insert = false;
                    }
                })
                .or_insert(n.val);

            if insert {
                candidates.push(Reverse(Node {
                    key: n.key,
                    val: *val,
                }));
            }
        }
    }
    true
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let input = &args[1];
    let data = fs::read_to_string(input).unwrap();
    let (_, v) = parse_file(&data).unwrap();
    let mut res: Vec<_> = (0..v.len()).into_par_iter().filter(|&n| solve(&v, n)).collect();
    res.sort();
    // needs some manual adjustment (swap order, subtract one from each)
    println!("{:?} ({})", v[res[0] - 1], res[0]);
}
