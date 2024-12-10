use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::{collections::BinaryHeap, fs};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
    height: u32,
    seen: bool,
}

impl Point {
    fn neighbors(&self) -> Vec<(usize, usize)> {
        let mut v = vec![(self.x + 1, self.y), (self.x, self.y + 1)];
        if let Some(x) = self.x.checked_add_signed(-1) {
            v.push((x, self.y));
        }
        if let Some(y) = self.y.checked_add_signed(-1) {
            v.push((self.x, y));
        }
        v
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, rhs: &Point) -> Option<std::cmp::Ordering> {
        self.height.partial_cmp(&rhs.height)
    }
}

impl Ord for Point {
    fn cmp(&self, rhs: &Self) -> std::cmp::Ordering {
        self.height.cmp(&rhs.height)
    }
}

fn parse_file(s: &str) -> Vec<Vec<Point>> {
    s.lines()
        .enumerate()
        .map(|(i, l)| {
            std::iter::repeat(i)
                .zip(l.chars().enumerate())
                .map(|(i, (j, h))| Point {
                    x: i,
                    y: j,
                    height: h.to_digit(10).unwrap(),
                    seen: false,
                })
                .collect()
        })
        .collect()
}

fn solve_one(mut data: Vec<Vec<Point>>, x: usize, y: usize) -> u64 {
    let mut total = 0;
    let mut heap = BinaryHeap::new();
    data[x][y].seen = true;
    heap.push(data[x][y]);

    while let Some(curr) = heap.pop() {
        if curr.height == 9 {
            total += 1;
            continue;
        }
        for (x, y) in curr.neighbors() {
            let Some(p) = data.get_mut(x).and_then(|d| d.get_mut(y)) else {
                continue;
            };
            if !p.seen && p.height == curr.height + 1 {
                p.seen = true;
                heap.push(*p);
            }
        }
    }

    total
}

fn solve(data: &Vec<Vec<Point>>) -> u64 {
    let starts: Vec<_> = data
        .iter()
        .flat_map(|v| v)
        .filter(|p| p.height == 0)
        .collect();
    starts
        .into_par_iter()
        .map(|p| {
            let data = data.clone();
            solve_one(data, p.x, p.y)
        })
        .sum()
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let input = &args[1];
    let data = fs::read_to_string(input).unwrap();
    let v = parse_file(&data);
    println!("{}", solve(&v));
}
