use nom::{
    bytes::complete::tag,
    character::complete::{self, char},
    error::{make_error, ErrorKind},
    sequence::{delimited, separated_pair},
    Err, IResult,
};
use std::fs;

fn parse_mul(s: &str) -> IResult<&str, u64> {
    let (s, _) = tag("mul")(s)?;
    let (s2, (a, b)) = delimited(
        char('('),
        separated_pair(complete::u64, char(','), complete::u64),
        char(')'),
    )(s)?;
    if a > 999 || b > 999 {
        return Err(Err::Error(make_error(s, ErrorKind::Fail)));
    }
    Ok((s2, a * b))
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let input = &args[1];
    let data = fs::read_to_string(input).unwrap();
    let mut s = data.as_str();
    let mut total = 0;
    while !s.is_empty() {
        if let Ok((new_s, n)) = parse_mul(s) {
            total += n;
            s = new_s;
        } else {
            s = &s[1..s.len()];
        }
    }
    println!("{total}");
}
