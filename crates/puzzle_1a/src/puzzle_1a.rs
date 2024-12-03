use std::fs;

fn calc_distance(left: &mut [i64], right: &mut [i64]) -> u64 {
    left.sort();
    right.sort();
    left.iter().zip(right.iter()).fold(0u64, |total, (l, r)| {
        total + l.abs_diff(*r)
    })
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let input = &args[1];
    let data = fs::read_to_string(input).unwrap();
    let ints: Vec<_> = data.split_ascii_whitespace().map(|s| str::parse::<i64>(s).unwrap()).collect();
    let (mut left, mut right): (Vec<_>, Vec<_>) = ints.chunks_exact(2).map(|s| (s[0], s[1])).unzip();
    println!("{}", calc_distance(&mut left, &mut right));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        let mut left = vec![3, 4, 2, 1, 3, 3];
        let mut right = vec![4, 3, 5, 3, 9, 3];
        assert_eq!(calc_distance(&mut left, &mut right), 11);
    }
}
