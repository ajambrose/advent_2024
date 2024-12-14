use nom::{
    bytes::complete::tag,
    character::complete::{self, char, line_ending},
    combinator::{all_consuming, eof},
    multi::{many0_count, many1},
    sequence::{preceded, separated_pair, terminated},
    IResult,
};
use rayon::prelude::*;
use std::{collections::HashSet, fs};

const X_WIDTH: i64 = 101;
const Y_WIDTH: i64 = 103;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: i64,
    y: i64,
}

struct Velocity {
    x: i64,
    y: i64,
}

struct Robot {
    p: Position,
    v: Velocity,
}

type RowData = Robot;

fn parse_line(s: &str) -> IResult<&str, RowData> {
    let (s, (px, py)) = preceded(
        tag("p="),
        separated_pair(complete::i64, char(','), complete::i64),
    )(s)?;
    let (s, (vx, vy)) = preceded(
        tag(" v="),
        separated_pair(complete::i64, char(','), complete::i64),
    )(s)?;
    Ok((
        s,
        Robot {
            p: Position { x: px, y: py },
            v: Velocity { x: vx, y: vy },
        },
    ))
}

fn parse_file(s: &str) -> IResult<&str, Vec<RowData>> {
    let (s, v) = many1(terminated(parse_line, line_ending))(s)?;
    let (s, _) = all_consuming(preceded(many0_count(line_ending), eof))(s)?;
    Ok((s, v))
}

fn solve(data: &[RowData]) {
    // 10,000 iterations picked by brute force
    let x: Vec<_> = (0..10000)
        .into_par_iter()
        .filter_map(|n| {
            let mut s = HashSet::new();
            let mut g = vec![vec!['.'; X_WIDTH as usize]; Y_WIDTH as usize];
            if data
                .iter()
                .map(|r| {
                    let mut x = (r.p.x + n * r.v.x) % X_WIDTH;
                    if x < 0 {
                        x += X_WIDTH;
                    }
                    let mut y = (r.p.y + n * r.v.y) % Y_WIDTH;
                    if y < 0 {
                        y += Y_WIDTH;
                    }
                    Position { x, y }
                })
                .all(|p| {
                    // Y and X reversed because it draws the picture in the correct orientation
                    g[p.y as usize][p.x as usize] = '#';
                    // any overlapping robots signals that this isn't a picture, so move on
                    // I'd be lying if I said I didn't get this hint from someone online,
                    // this is not exactly a fair intuition
                    s.insert(p)
                })
            {
                // no robots overlap
                Some((n, g))
            } else {
                // at least one overlap
                None
            }
        })
        .collect();

    // "carefully examine" all printed results visually and pick the answer based on that
    // (there's only one that satisfies the HashSet heuristic)
    for (n, g) in x {
        println!("{n}:");
        for r in g {
            println!("{}", r.iter().collect::<String>());
        }
    }
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let input = &args[1];
    let data = fs::read_to_string(input).unwrap();
    let (_, v) = parse_file(&data).unwrap();
    solve(&v);
}
