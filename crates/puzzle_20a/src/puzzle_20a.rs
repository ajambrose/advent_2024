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

fn cheat_neighbors(x: usize, y: usize) -> [(usize, usize); 8] {
    [
        (x - 2, y),
        (x + 2, y),
        (x, y - 2),
        (x, y + 2),
        (x - 1, y - 1),
        (x - 1, y + 1),
        (x + 1, y - 1),
        (x + 1, y + 1),
    ]
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

    let mut total = 0;
    for (x, y) in steps.into_iter().rev() {
        let v = track[x][y];
        assert_ne!(v, 0);
        for (x1, y1) in cheat_neighbors(x, y) {
            let v1 = track[x1][y1];
            if v1 == -1 {
                continue;
            }
            assert_ne!(v1, 0);

            if v - v1 - 2 >= 100 {
                total += 1;
            }
        }
    }

    total
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let input = &args[1];
    let data = fs::read_to_string(input).unwrap();
    let mut track = parse_file(&data);
    println!("{}", solve(&mut track));
}
