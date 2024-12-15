use std::fs;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Space {
    Empty,
    Wall,
    Box,
    Robot,
}

type Point = (usize, usize);

fn parse_map(s: &str) -> (Point, Vec<Vec<Space>>) {
    let mut pos = None;
    let r = s
        .lines()
        .enumerate()
        .map(|(i, l)| {
            l.chars()
                .enumerate()
                .map(|(j, c)| match c {
                    '.' => Space::Empty,
                    '#' => Space::Wall,
                    'O' => Space::Box,
                    '@' => {
                        assert!(pos.is_none());
                        pos = Some((i, j));
                        Space::Robot
                    }
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();
    (pos.unwrap(), r)
}

fn parse_dirs(s: &str) -> Vec<(isize, isize)> {
    s.lines()
        .flat_map(|l| l.chars())
        .map(|c| match c {
            '<' => (0, -1),
            '>' => (0, 1),
            '^' => (-1, 0),
            'v' => (1, 0),
            _ => unreachable!(),
        })
        .collect()
}

fn parse_file(s: &str) -> ((Point, Vec<Vec<Space>>), Vec<(isize, isize)>) {
    let (map, dirs) = s.split_once("\r\n\r\n").unwrap();
    (parse_map(map), parse_dirs(dirs))
}

#[allow(unused)]
fn print_map(map: &Vec<Vec<Space>>) {
    let x: Vec<_> = map
        .iter()
        .map(|r| {
            r.iter()
                .map(|s| match s {
                    Space::Empty => '.',
                    Space::Wall => '#',
                    Space::Box => 'O',
                    Space::Robot => '@',
                })
                .collect::<String>()
        })
        .collect();
    for l in x {
        println!("{l}");
    }
}

fn calculate_gps(map: &Vec<Vec<Space>>) -> usize {
    map.iter()
        .enumerate()
        .flat_map(|(i, r)| std::iter::repeat(i).zip(r.iter().enumerate()))
        .filter_map(|(i, (j, _))| {
            if map[i][j] == Space::Box {
                Some(100 * i + j)
            } else {
                None
            }
        })
        .sum()
}

fn swap_spaces(map: &mut Vec<Vec<Space>>, a: Point, b: Point) {
    let tmp = map[a.0][a.1];
    map[a.0][a.1] = map[b.0][b.1];
    map[b.0][b.1] = tmp;
}

fn solve(mut curr: Point, map: &mut Vec<Vec<Space>>, dirs: &[(isize, isize)]) -> usize {
    'outer: for dir in dirs {
        assert!(map[curr.0][curr.1] == Space::Robot);
        let next = (
            curr.0.checked_add_signed(dir.0).unwrap(),
            curr.1.checked_add_signed(dir.1).unwrap(),
        );

        match map[next.0][next.1] {
            Space::Empty => swap_spaces(map, curr, next),
            Space::Wall => continue 'outer,
            Space::Box => {
                let mut gen = next;
                for s in std::iter::from_fn(move || {
                    gen = (
                        gen.0.checked_add_signed(dir.0).unwrap(),
                        gen.1.checked_add_signed(dir.1).unwrap(),
                    );
                    Some(gen)
                }) {
                    match map[s.0][s.1] {
                        Space::Empty => {
                            swap_spaces(map, s, next);
                            swap_spaces(map, curr, next);
                            break;
                        }
                        Space::Wall => continue 'outer,
                        Space::Box => (),
                        _ => unreachable!(),
                    }
                }
            }
            _ => unreachable!(),
        }
        curr = next;
    }
    calculate_gps(&map)
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let input = &args[1];
    let data = fs::read_to_string(input).unwrap();
    let ((start, mut map), dirs) = parse_file(&data);
    println!("{}", solve(start, &mut map, &dirs));
}
