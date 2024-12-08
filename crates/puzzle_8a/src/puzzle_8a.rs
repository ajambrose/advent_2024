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

fn antinodes(p: &PointPair) -> Vec<Point> {
    let x = (p.0 .0 as isize) - (p.1 .0 as isize);
    let y = (p.0 .1 as isize) - (p.1 .1 as isize);

    let mut r = vec![];
    if let Some(p1) =
        p.0 .0
            .checked_add_signed(x)
            .and_then(|x| Some((x, p.0 .1.checked_add_signed(y)?)))
    {
        r.push(p1);
    }
    if let Some(p2) =
        p.1 .0
            .checked_add_signed(-x)
            .and_then(|x| Some((x, p.1 .1.checked_add_signed(-y)?)))
    {
        r.push(p2);
    }

    r
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
            for n in antinodes(&p) {
                if n.0 < data.len() && n.1 < data[0].len() {
                    nodes.insert(n);
                }
            }
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_each_pair() {
        let v = vec![(1usize, 2usize), (3, 4), (5, 6), (7, 8)];
        let v2: Vec<_> = each_pair(&v).collect();
        assert_eq!(
            v2,
            vec![
                ((1, 2), (3, 4)),
                ((1, 2), (5, 6)),
                ((1, 2), (7, 8)),
                ((3, 4), (5, 6)),
                ((3, 4), (7, 8)),
                ((5, 6), (7, 8))
            ]
        );
    }
}
