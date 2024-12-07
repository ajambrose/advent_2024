use nom::{
    character::complete::{self, char, line_ending, space1},
    combinator::{all_consuming, eof},
    multi::{many0_count, many1},
    sequence::{preceded, terminated, tuple},
    IResult,
};
use std::fs;

type RowData = (u64, Vec<u64>);

fn parse_line(s: &str) -> IResult<&str, RowData> {
    let (s, row) = terminated(
        tuple((
            terminated(complete::u64, char(':')),
            many1(preceded(space1, complete::u64)),
        )),
        line_ending,
    )(s)?;
    Ok((s, row.into()))
}

fn parse_file(s: &str) -> IResult<&str, Vec<RowData>> {
    let (s, v) = many1(parse_line)(s)?;
    let (s, _) = all_consuming(preceded(many0_count(line_ending), eof))(s)?;
    Ok((s, v))
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Op {
    Add,
    Mul,
}

impl Op {
    pub fn call(&self, a: u64, b: u64) -> u64 {
        match self {
            Op::Add => a + b,
            Op::Mul => a * b,
        }
    }

    pub fn next(arr: &mut [Op]) -> bool {
        for op in arr {
            if *op == Op::Add {
                *op = Op::Mul;
                return true;
            } else {
                *op = Op::Add;
            }
        }

        false
    }
}

fn solve(data: &[(u64, Vec<u64>)]) -> u64 {
    let mut total = 0;
    for (res, nums) in data {
        let mut ops = vec![Op::Add; nums.len() - 1];
        loop {
            let output = nums
                .iter()
                .skip(1)
                .zip(&ops)
                .fold(nums[0], |t, (&n, &op)| op.call(t, n));
            if output == *res {
                total += output;
                break;
            }
            if !Op::next(&mut ops) {
                break;
            }
        }
    }
    total
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let input = &args[1];
    let data = fs::read_to_string(input).unwrap();
    let (_, v) = parse_file(&data).unwrap();
    println!("{}", solve(&v));
}
