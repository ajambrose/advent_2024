use std::fs;

fn is_unsafe_move(greater: i64, lesser: i64) -> bool {
    let diff = greater - lesser;
    diff < 1 || diff > 3
}

fn is_safe(report: &[i64]) -> bool {
    if report.len() == 1 {
        return true;
    }

    if report[0] < report[1] {
        // increasing
        for i in 1..report.len() {
            if is_unsafe_move(report[i], report[i-1]) {
                return false;
            }
        }
        return true;
    } else {
        // decreasing
        for i in 1..report.len() {
            if is_unsafe_move(report[i-1], report[i]) {
                return false;
            }
        }
        return true;
    }
}

fn count_safe(reports: &[Vec<i64>]) -> u64 {
    reports
        .iter()
        .fold(0, |total, level| total + is_safe(level) as u64)
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let input = &args[1];
    let data = fs::read_to_string(input).unwrap();
    let report: Vec<_> = data
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|s| s.parse::<i64>().unwrap())
                .collect()
        })
        .collect();
    println!("{}", count_safe(&report));
}
