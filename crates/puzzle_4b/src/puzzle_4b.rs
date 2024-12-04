use nom::{
    character::complete::{alpha1, line_ending},
    combinator::{all_consuming, eof},
    multi::{many0_count, many1},
    sequence::{preceded, terminated},
    IResult,
};
use std::fs;

type RowData = String;

fn parse_line(s: &str) -> IResult<&str, RowData> {
    let (s, row) = terminated(alpha1, line_ending)(s)?;
    Ok((s, row.into()))
}

fn parse_file(s: &str) -> IResult<&str, Vec<RowData>> {
    let (s, v) = many1(parse_line)(s)?;
    let (s, _) = all_consuming(preceded(many0_count(line_ending), eof))(s)?;
    Ok((s, v))
}

fn at(text: &[String], i: usize, j: usize) -> char {
    text[i].chars().nth(j).unwrap()
}

fn is_xmas(text: &[String], i: usize, j: usize) -> bool {
    if (at(text, i - 1, j - 1) == 'M' && at(text, i + 1, j + 1) == 'S')
        || (at(text, i - 1, j - 1) == 'S' && at(text, i + 1, j + 1) == 'M')
    {
        if (at(text, i - 1, j + 1) == 'M' && at(text, i + 1, j - 1) == 'S')
            || (at(text, i - 1, j + 1) == 'S' && at(text, i + 1, j - 1) == 'M')
        {
            return true;
        }
    }
    false
}

fn find_xmas(text: &[String]) -> u64 {
    let mut total = 0;
    let str_len = text[0].len();
    for (i, s) in text.iter().enumerate() {
        if i == 0 || i == text.len() - 1 {
            continue;
        }
        for (j, c) in s.chars().enumerate() {
            if j == 0 || j == str_len - 1 {
                continue;
            }
            if c != 'A' {
                continue;
            }
            total += is_xmas(text, i, j) as u64;
        }
    }
    total
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let input = &args[1];
    let data = fs::read_to_string(input).unwrap();
    let (_, v) = parse_file(&data).unwrap();
    println!("{}", find_xmas(&v));
}
