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

    fn diagonals(&self) -> Vec<(usize, usize)> {
        let mut v = vec![(self.x + 1, self.y + 1)];
        if let Some(x) = self.x.checked_add_signed(-1) {
            v.push((x, self.y + 1));
            if let Some(y) = self.y.checked_add_signed(-1) {
                v.push((x, y));
            }
        }
        if let Some(y) = self.y.checked_add_signed(-1) {
            v.push((self.x + 1, y));
        }
        v
    }

    fn in_line(&self, rhs: &Self) -> bool {
        self.x == rhs.x || self.y == rhs.y
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

fn at(data: &Vec<Vec<Point>>, x: usize, y: usize) -> Option<&Point> {
    data.get(x).and_then(|d| d.get(y))
}

fn at_mut(data: &mut Vec<Vec<Point>>, x: usize, y: usize) -> Option<&mut Point> {
    data.get_mut(x).and_then(|d| d.get_mut(y))
}

fn count_corners(data: &Vec<Vec<Point>>, curr: Point, nbrs: &[Point]) -> u64 {
    match nbrs.len() {
        0 => return 4,
        1 => return 2,
        2 => {
            if nbrs[0].in_line(&nbrs[1]) {
                return 0;
            }

            let x = if curr.x == nbrs[0].x {
                nbrs[1].x
            } else {
                nbrs[0].x
            };

            let y = if curr.y == nbrs[0].y {
                nbrs[1].y
            } else {
                nbrs[0].y
            };

            if data[x][y].c == curr.c {
                1
            } else {
                2
            }
        }
        3 => {
            let idx: usize = if nbrs[0].in_line(&nbrs[1]) {
                2
            } else if nbrs[0].in_line(&nbrs[2]) {
                1
            } else {
                0
            };

            let n = nbrs[idx];
            let mut ret = 0;
            if n.x == curr.x {
                if data[n.x + 1][n.y].c != curr.c {
                    ret += 1;
                }
                if data[n.x - 1][n.y].c != curr.c {
                    ret += 1;
                }
            } else {
                if data[n.x][n.y + 1].c != curr.c {
                    ret += 1;
                }
                if data[n.x][n.y - 1].c != curr.c {
                    ret += 1;
                }
            }
            ret
        }
        4 => {
            let mut ret = 0;
            for (x, y) in curr.diagonals() {
                if data[x][y].c != curr.c {
                    ret += 1;
                }
            }
            ret
        }
        _ => unreachable!(),
    }
}

fn calc_plot_cost(data: &mut Vec<Vec<Point>>, start: Point) -> u64 {
    let mut area = 0;
    let mut sides = 0;
    let mut candidates = vec![start];

    while let Some(curr) = candidates.pop() {
        area += 1;
        let ns = curr.neighbors();
        for &(x, y) in &ns {
            let Some(p) = at_mut(data, x, y) else {
                continue;
            };

            if p.c == curr.c {
                if !p.seen {
                    p.seen = true;
                    candidates.push(*p);
                }
            }
        }

        let nbrs: Vec<_> = ns
            .into_iter()
            .filter_map(|(x, y)| {
                let pt = at(data, x, y)?;
                if pt.c == curr.c {
                    Some(pt)
                } else {
                    None
                }
            })
            .copied()
            .collect();

        sides += count_corners(data, curr, &nbrs);
    }

    area * sides
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
