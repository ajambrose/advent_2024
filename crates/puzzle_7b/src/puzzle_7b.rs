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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Op {
    Add,
    Mul,
    Or,
}

impl Op {
    pub fn call(&self, a: u64, b: u64) -> u64 {
        match self {
            Op::Add => a.checked_add(b).unwrap(),
            Op::Mul => a.checked_mul(b).unwrap(),
            Op::Or => (a.checked_mul(10u64.pow(1 + (b as f64).log10().trunc() as u32)))
                .unwrap()
                .checked_add(b)
                .unwrap(),
        }
    }

    pub fn next(arr: &mut [Op]) -> bool {
        for op in arr {
            match op {
                Op::Add => {
                    *op = Op::Mul;
                    return true;
                }
                Op::Mul => {
                    *op = Op::Or;
                    return true;
                }
                Op::Or => {
                    *op = Op::Add;
                }
            }
        }

        false
    }
}

fn solve(data: &[(u64, Vec<u64>)]) -> u64 {
    let mut total: u64 = 0;
    for (res, nums) in data {
        let mut ops = vec![Op::Add; nums.len() - 1];
        loop {
            let output = nums
                .iter()
                .skip(1)
                .zip(&ops)
                .fold(nums[0], |t, (&n, &op)| op.call(t, n));
            if output == *res {
                total = total.checked_add(output).unwrap();
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
