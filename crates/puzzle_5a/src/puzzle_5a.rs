use std::collections::{HashMap, HashSet};
use std::fs;

fn parse_file(file: &str) -> (HashMap<u64, HashSet<u64>>, Vec<Vec<u64>>) {
    let mut lines = file.lines();
    let v: Vec<_> = lines
        .by_ref()
        .take_while(|l| !l.is_empty())
        .map(|l| l.split_once('|').unwrap())
        .map(|(n, m)| (n.parse::<u64>().unwrap(), m.parse::<u64>().unwrap()))
        .collect();
    let mut m: HashMap<u64, HashSet<u64>> = HashMap::new();
    for (a, b) in v {
        m.entry(a).or_default().insert(b);
    }
    let v: Vec<_> = lines
        .map(|l| {
            l.split(',')
                .map(str::parse::<u64>)
                .collect::<Result<_, _>>()
                .unwrap()
        })
        .collect();
    (m, v)
}

fn solve(rules: &HashMap<u64, HashSet<u64>>, pages: &Vec<Vec<u64>>) -> u64 {
    let mut total = 0;
    'page: for page in pages {
        for (i, n) in page.iter().enumerate() {
            if let Some(rule) = rules.get(n) {
                for j in (0..i).rev() {
                    if rule.get(&page[j]).is_some() {
                        continue 'page;
                    }
                }
            }
        }
        total += page[page.len() / 2];
    }
    total
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let input = &args[1];
    let data = fs::read_to_string(input).unwrap();
    let (m, s) = parse_file(&data);
    println!("{}", solve(&m, &s));
}
