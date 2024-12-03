use std::{
    collections::{hash_map::Entry, HashMap},
    fs,
};

fn calc_similarity(left: &mut [i64], right: &HashMap<i64, u64>) -> u64 {
    left.iter().fold(0, |total, &n| {
        total + (n as u64 * right.get(&n).copied().unwrap_or(0))
    })
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let input = &args[1];
    let data = fs::read_to_string(input).unwrap();
    let ints: Vec<_> = data
        .split_ascii_whitespace()
        .map(|s| str::parse::<i64>(s).unwrap())
        .collect();
    let mut left = vec![];
    let mut right = HashMap::new();
    for s in ints.chunks_exact(2) {
        left.push(s[0]);
        let v = match right.entry(s[1]) {
            Entry::Vacant(e) => e.insert(0u64),
            Entry::Occupied(e) => e.into_mut(),
        };
        *v += 1;
    }
    println!("{}", calc_similarity(&mut left, &right));
}
