use nom::{
    bytes::complete::tag,
    character::complete::{self, char, line_ending},
    combinator::{all_consuming, eof},
    multi::{many0_count, many1},
    sequence::{preceded, separated_pair, terminated},
    IResult,
};
use std::fs;

const X_WIDTH: i64 = 101;
const X_HALF: i64 = X_WIDTH / 2;
const Y_WIDTH: i64 = 103;
const Y_HALF: i64 = Y_WIDTH / 2;

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

fn solve(data: &[RowData]) -> u64 {
    let mut totals = [0u64; 4];
    data.iter()
        .filter_map(|r| {
            let mut x = (r.p.x + 100 * r.v.x) % X_WIDTH;
            if x < 0 {
                x += X_WIDTH;
            }
            let mut y = (r.p.y + 100 * r.v.y) % Y_WIDTH;
            if y < 0 {
                y += Y_WIDTH;
            }
            if x == X_HALF || y == Y_HALF {
                None
            } else {
                Some(Position { x, y })
            }
        })
        .for_each(|p| {
            let idx = if p.x < X_HALF {
                if p.y < Y_HALF {
                    0
                } else {
                    1
                }
            } else {
                if p.y < Y_HALF {
                    2
                } else {
                    3
                }
            };
            totals[idx] += 1;
        });
    totals.iter().copied().reduce(|acc, n| acc * n).unwrap()
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let input = &args[1];
    let data = fs::read_to_string(input).unwrap();
    let (_, v) = parse_file(&data).unwrap();
    println!("{}", solve(&v));
}
