use hashbrown::HashMap;
use std::fs;

fn parse_file(s: &str) -> Vec<u64> {
    s.lines()
        .flat_map(|l| l.split(' ').map(str::parse::<u64>))
        .collect::<Result<_, _>>()
        .unwrap()
}

fn next_n(d: u64) -> (u64, Option<u64>) {
    if d == 0 {
        (1, None)
    } else {
        let n = (d as f64).log10().trunc() as u32 + 1;
        if n & 1 == 1 {
            (d * 2024, None)
        } else {
            let base = 10u64.pow(n / 2);
            let r = d / base;
            let m = d % base;
            (r, Some(m))
        }
    }
}

fn get_n_at_iter(d: u64, base: u64, m: &mut HashMap<(u64, u64), usize>) -> usize {
    if base == 0 {
        return 1;
    }

    if let Some(&res) = m.get(&(d, base)) {
        return res;
    }

    let (a, b) = next_n(d);
    let mut res = get_n_at_iter(a, base - 1, m);
    if let Some(b) = b {
        res += get_n_at_iter(b, base - 1, m);
    }
    m.insert((d, base), res);
    res
}

fn solve(data: &[u64]) -> usize {
    let mut m: HashMap<(u64, u64), usize> = HashMap::new();
    let mut total = 0;
    for &d in data {
        total += get_n_at_iter(d, 75, &mut m);
    }
    total
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let input = &args[1];
    let data = fs::read_to_string(input).unwrap();
    let v = parse_file(&data);
    println!("{}", solve(&v));
}
