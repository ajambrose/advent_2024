use std::fs;

#[derive(Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
    c: char,
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

fn parse_file(s: &str) -> Vec<Vec<Point>> {
    s.lines()
        .enumerate()
        .map(|(i, l)| {
            std::iter::repeat(i)
                .zip(l.chars().enumerate())
                .map(|(x, (y, c))| Point {
                    x,
                    y,
                    c,
                    seen: false,
                })
                .collect()
        })
        .collect()
}

fn calc_plot_cost(data: &mut Vec<Vec<Point>>, start: Point) -> u64 {
    let mut area = 0;
    let mut per = 0;
    let mut candidates = vec![start];

    while let Some(curr) = candidates.pop() {
        area += 1;
        per += 4;
        for (x, y) in curr.neighbors() {
            let Some(p) = data.get_mut(x).and_then(|d| d.get_mut(y)) else {
                continue;
            };
            if p.c == curr.c {
                per -= 1;
                if !p.seen {
                    p.seen = true;
                    candidates.push(*p);
                }
            }
        }
    }

    area * per
}

fn solve(data: &mut Vec<Vec<Point>>) -> u64 {
    let mut cost: u64 = 0;
    for i in 0..data.len() {
        for j in 0..data[0].len() {
            if !data[i][j].seen {
                data[i][j].seen = true;
                cost += calc_plot_cost(data, data[i][j]);
            }
        }
    }
    cost
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let input = &args[1];
    let data = fs::read_to_string(input).unwrap();
    let mut v = parse_file(&data);
    println!("{}", solve(&mut v));
}
