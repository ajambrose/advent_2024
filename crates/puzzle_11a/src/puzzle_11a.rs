use either::Either;
use std::fs;

fn parse_file(s: &str) -> Vec<u64> {
    s.lines()
        .flat_map(|l| l.split(' ').map(str::parse::<u64>))
        .collect::<Result<_, _>>()
        .unwrap()
}

fn blink(data: &[u64]) -> Vec<u64> {
    data.iter()
        .flat_map(|&d| {
            if d == 0 {
                Either::Left(std::iter::once(1))
            } else {
                let n = (d as f64).log10().trunc() as u32 + 1;
                if n & 1 == 1 {
                    Either::Left(std::iter::once(d * 2024))
                } else {
                    let base = 10u64.pow(n / 2);
                    let r = d / base;
                    let m = d % base;
                    Either::Right(std::iter::once(r).chain(std::iter::once(m)))
                }
            }
        })
        .collect()
}

fn solve(mut data: Vec<u64>) -> usize {
    for _ in 0..25 {
        data = blink(&data);
    }
    data.len()
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let input = &args[1];
    let data = fs::read_to_string(input).unwrap();
    let v = parse_file(&data);
    println!("{}", solve(v));
}
