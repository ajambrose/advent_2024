use std::fs;

fn is_unsafe_move(greater: i64, lesser: i64) -> bool {
    let diff = greater - lesser;
    diff < 1 || diff > 3
}

fn is_safe(report: &[i64]) -> bool {
    is_safe_impl(report, true)
}

fn is_safe_retry(report: &[i64]) -> bool {
    is_safe_impl(&report[1..report.len()], false)
}

fn is_safe_impl(report: &[i64], mulligan: bool) -> bool {
    if report.len() == 1 {
        return true;
    }

    if report[0] < report[1] {
        // increasing
        is_safe_increasing(report, mulligan) || is_safe_decreasing(report, mulligan)
    } else {
        // decreasing
        is_safe_decreasing(report, mulligan) || is_safe_increasing(report, mulligan)
    }
}

fn is_safe_increasing(report: &[i64], mut mulligan: bool) -> bool {
    let mut i = 1;
    while i < report.len() {
        if is_unsafe_move(report[i], report[i - 1]) {
            if mulligan {
                mulligan = false;
                if i == report.len() - 1 {
                    break;
                }
                if !is_unsafe_move(report[i + 1], report[i - 1]) {
                    i += 2;
                    continue;
                }
                if i > 1 && !is_unsafe_move(report[i], report[i - 2]) {
                    i += 1;
                    continue;
                }
            }
            return false;
        }
        i += 1;
    }
    true
}

fn is_safe_decreasing(report: &[i64], mut mulligan: bool) -> bool {
    let mut i = 1;
    while i < report.len() {
        if is_unsafe_move(report[i - 1], report[i]) {
            if mulligan {
                mulligan = false;
                if i == report.len() - 1 {
                    break;
                }
                if !is_unsafe_move(report[i - 1], report[i + 1]) {
                    i += 2;
                    continue;
                }
                if i > 1 && !is_unsafe_move(report[i - 2], report[i]) {
                    i += 1;
                    continue;
                }
            }
            return false;
        }
        i += 1;
    }
    true
}

fn count_safe(reports: &[Vec<i64>]) -> u64 {
    reports.iter().fold(0, |total, level| {
        if is_safe(level) {
            total + 1
        } else if is_safe_retry(level) {
            total + 1
        } else {
            total
        }
    })
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
