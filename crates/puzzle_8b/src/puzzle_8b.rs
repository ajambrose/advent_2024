use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn parse_file(s: &str) -> Vec<Vec<char>> {
    s.lines().map(|l| l.chars().collect()).collect()
}

type Point = (usize, usize);
type PointPair = (Point, Point);

fn each_pair<'a>(data: &'a [Point]) -> impl Iterator<Item = PointPair> + 'a {
    data.iter()
        .copied()
        .enumerate()
        .map(|(i, p)| std::iter::repeat(p).zip(data.iter().copied().skip(i + 1)))
        .flatten()
}

fn distance(p: &PointPair) -> (isize, isize) {
    let x = (p.0 .0 as isize) - (p.1 .0 as isize);
    let y = (p.0 .1 as isize) - (p.1 .1 as isize);

    (x, y)
}

fn generate_antinodes(mut curr: Point, d: (isize, isize)) -> impl Iterator<Item = Point> {
    std::iter::once(curr).chain(std::iter::from_fn(move || {
        curr.0 = curr.0.checked_add_signed(d.0)?;
        curr.1 = curr.1.checked_add_signed(d.1)?;
        Some(curr)
    }))
}

fn solve(data: &[Vec<char>]) -> u64 {
    let mut points: HashMap<char, Vec<Point>> = HashMap::new();
    data.iter()
        .enumerate()
        .map(|(i, v)| std::iter::repeat(i).zip(v.iter().enumerate().filter(|(_, &c)| c != '.')))
        .flatten()
        .for_each(|(i, (j, c))| {
            points.entry(*c).or_insert(vec![]).push((i, j));
        });

    let mut nodes = HashSet::new();

    for (_, v) in points {
        for p in each_pair(&v) {
            let d = distance(&p);
            generate_antinodes(p.0, d)
                .take_while(|p| p.0 < data.len() && p.1 < data[0].len())
                .for_each(|p| {
                    nodes.insert(p);
                });
            generate_antinodes(p.1, (-d.0, -d.1))
                .take_while(|p| p.0 < data.len() && p.1 < data[0].len())
                .for_each(|p| {
                    nodes.insert(p);
                });
        }
    }

    nodes.len() as u64
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let input = &args[1];
    let data = fs::read_to_string(input).unwrap();
    let v = parse_file(&data);
    println!("{}", solve(&v));
}
