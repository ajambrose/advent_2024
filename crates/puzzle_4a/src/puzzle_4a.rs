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

fn enumerate_idx(idx: usize, max: usize) -> Vec<[usize; 3]> {
    let mut res = vec![];
    if idx > 2 {
        res.push([idx - 1, idx - 2, idx - 3]);
    }
    if idx < max - 3 {
        res.push([idx + 1, idx + 2, idx + 3]);
    }
    res
}

fn enumerate_idxs(i: usize, j: usize, max: usize) -> Vec<Vec<(usize, usize)>> {
    let mut res = vec![];
    let i_idxs = enumerate_idx(i, max);
    let j_idxs = enumerate_idx(j, max);
    for a in &i_idxs {
        res.push(a.iter().copied().zip([j, j, j].into_iter()).collect());
        for b in &j_idxs {
            res.push(a.iter().copied().zip(b.iter().copied()).collect());
        }
    }
    for b in &j_idxs {
        res.push([i, i, i].into_iter().zip(b.iter().copied()).collect());
    }
    res
}

fn is_xmas(text: &[String], idxs: &[(usize, usize)]) -> bool {
    let xmas = ['M', 'A', 'S'];
    idxs.iter()
        .zip(xmas.iter())
        .all(|(&(i, j), &c)| text[i].chars().nth(j).unwrap() == c)
}

fn find_xmas(text: &[String]) -> u64 {
    let mut total = 0;
    let str_len = text[0].len();
    for (i, s) in text.iter().enumerate() {
        for (j, c) in s.chars().enumerate() {
            if c != 'X' {
                continue;
            }
            let idx_list = enumerate_idxs(i, j, str_len);
            for idxs in idx_list {
                total += is_xmas(text, idxs.as_slice()) as u64;
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
    println!("{}", find_xmas(&v));
}
