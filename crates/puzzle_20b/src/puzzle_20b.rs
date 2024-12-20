use std::fs;

fn parse_file(s: &str) -> Vec<Vec<i64>> {
    s.lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '#' => -1,
                    '.' => 0,
                    'S' => 1,
                    'E' => i64::MAX,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect()
}

fn find_start(track: &Vec<Vec<i64>>) -> (usize, usize) {
    for (i, r) in track.iter().enumerate() {
        for (j, &c) in r.iter().enumerate() {
            if c == 1 {
                return (i, j);
            }
        }
    }
    unreachable!()
}

fn neighbors(x: usize, y: usize) -> [(usize, usize); 4] {
    [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
}

fn dist(a: (usize, usize), b: (usize, usize)) -> usize {
    a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
}

fn solve(track: &mut Vec<Vec<i64>>) -> u64 {
    track.insert(0, vec![-1; track[0].len()]);
    track.push(vec![-1; track[0].len()]);
    track.iter_mut().for_each(|r| {
        r.insert(0, -1);
        r.push(-1);
    });
    let (mut x, mut y) = find_start(&track);
    let mut steps = vec![];
    let mut curr = 2i64;
    'outer: loop {
        steps.push((x, y));
        for (x1, y1) in neighbors(x, y) {
            if track[x1][y1] == 0 {
                track[x1][y1] = curr;
                curr += 1;
                x = x1;
                y = y1;
                continue 'outer;
            } else if track[x1][y1] == i64::MAX {
                x = x1;
                y = y1;
                track[x][y] = curr;
                steps.push((x, y));
                break 'outer;
            }
        }
        unreachable!("{x}, {y} is {}", track[x][y])
    }

    steps
        .into_iter()
        .map(|(x, y)| {
            let v = track[x][y];
            assert_ne!(v, 0);

            (x.saturating_sub(20)..=x + 20)
                .into_iter()
                .filter_map(|x1| track.get(x1).map(|r| (x1, r)))
                .flat_map(|(x1, r)| {
                    (y.saturating_sub(20)..=y + 20)
                        .into_iter()
                        .filter_map(move |y1| r.get(y1).copied().map(|v1| ((x1, y1), v1)))
                })
                .filter_map(|(p1, v1)| {
                    let d = dist((x, y), p1);
                    let saved = v - v1 - d as i64;
                    if v1 != -1 && d <= 20 && saved >= 100 {
                        Some(1)
                    } else {
                        None
                    }
                })
                .sum::<u64>() // how to break HM in 23 easy lines
        })
        .sum()
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let input = &args[1];
    let data = fs::read_to_string(input).unwrap();
    let mut track = parse_file(&data);
    println!("{}", solve(&mut track));
}
