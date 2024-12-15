use std::{
    collections::{BinaryHeap, HashSet},
    fs,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Space {
    Empty,
    Wall,
    LeftBox,
    RightBox,
    Robot,
}

type Point = (usize, usize);

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct SpaceSwap {
    a: Point,
    b: Point,
    depth: usize,
}

impl PartialOrd for SpaceSwap {
    fn partial_cmp(&self, rhs: &SpaceSwap) -> Option<std::cmp::Ordering> {
        self.depth.partial_cmp(&rhs.depth)
    }
}

impl Ord for SpaceSwap {
    fn cmp(&self, rhs: &SpaceSwap) -> std::cmp::Ordering {
        self.depth.cmp(&rhs.depth)
    }
}

fn parse_map(s: &str) -> (Point, Vec<Vec<Space>>) {
    use std::iter::once;
    let mut pos = None;
    let map: Vec<Vec<_>> = s
        .lines()
        .map(|l| {
            l.chars()
                .flat_map(|c| match c {
                    '.' => once(Space::Empty).chain(once(Space::Empty)),
                    '#' => once(Space::Wall).chain(once(Space::Wall)),
                    'O' => once(Space::LeftBox).chain(once(Space::RightBox)),
                    '@' => once(Space::Robot).chain(once(Space::Empty)),
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();

    // too lazy to handle all the adjustments for horizontal enumeration above and the problem space is small
    for (i, r) in map.iter().enumerate() {
        for (j, &s) in r.iter().enumerate() {
            if s == Space::Robot {
                assert!(pos.is_none());
                pos = Some((i, j));
            }
        }
    }
    (pos.unwrap(), map)
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
    // windows lol
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
                    Space::LeftBox => '[',
                    Space::RightBox => ']',
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
        .flat_map(|(i, r)| std::iter::repeat(i).zip((0..r.len()).into_iter()))
        .filter_map(|(i, j)| {
            if map[i][j] == Space::LeftBox {
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

fn move_boxes(
    map: &Vec<Vec<Space>>,
    stack: &mut HashSet<SpaceSwap>,
    depth: usize,
    left: Point,
    right: Point,
    dir: (isize, isize),
) -> bool {
    let next_left = (
        left.0.checked_add_signed(dir.0).unwrap(),
        left.1.checked_add_signed(dir.1).unwrap(),
    );
    let next_right = (
        right.0.checked_add_signed(dir.0).unwrap(),
        right.1.checked_add_signed(dir.1).unwrap(),
    );

    if map[next_left.0][next_left.1] == Space::Wall
        || map[next_right.0][next_right.1] == Space::Wall
    {
        return false;
    }

    if map[next_left.0][next_left.1] == Space::Empty
        && map[next_right.0][next_right.1] == Space::Empty
    {
        stack.insert(SpaceSwap {
            a: left,
            b: next_left,
            depth,
        });
        stack.insert(SpaceSwap {
            a: right,
            b: next_right,
            depth,
        });
        return true;
    }

    match map[next_left.0][next_left.1] {
        Space::LeftBox => {
            assert!(map[next_right.0][next_right.1] == Space::RightBox);
            stack.insert(SpaceSwap {
                a: left,
                b: next_left,
                depth,
            });
            stack.insert(SpaceSwap {
                a: right,
                b: next_right,
                depth,
            });
            return move_boxes(map, stack, depth + 1, next_left, next_right, dir);
        }
        Space::RightBox => {
            stack.insert(SpaceSwap {
                a: left,
                b: next_left,
                depth,
            });
            let next_right = next_left;
            let next_left = (next_left.0, next_left.1 - 1);
            if !move_boxes(map, stack, depth + 1, next_left, next_right, dir) {
                return false;
            }
        }
        Space::Empty => {
            stack.insert(SpaceSwap {
                a: left,
                b: next_left,
                depth,
            });
        }
        _ => unreachable!(),
    }

    if map[next_right.0][next_right.1] == Space::LeftBox {
        stack.insert(SpaceSwap {
            a: right,
            b: next_right,
            depth,
        });
        let next_left = next_right;
        let next_right = (next_right.0, next_right.1 + 1);
        move_boxes(map, stack, depth + 1, next_left, next_right, dir)
    } else if map[next_right.0][next_right.1] == Space::Empty {
        stack.insert(SpaceSwap {
            a: right,
            b: next_right,
            depth,
        });
        true
    } else {
        unreachable!()
    }
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
            Space::LeftBox | Space::RightBox => {
                if dir.0 == 0 {
                    let mut stack = vec![curr, next];
                    let mut gen = next;
                    for s in std::iter::from_fn(move || {
                        gen = (
                            gen.0.checked_add_signed(dir.0).unwrap(),
                            gen.1.checked_add_signed(dir.1).unwrap(),
                        );
                        Some(gen)
                    }) {
                        stack.push((s.0, s.1));
                        match map[s.0][s.1] {
                            Space::Empty => break,
                            Space::Wall => continue 'outer,
                            Space::Robot => unreachable!(),
                            _ => (),
                        }
                    }
                    let mut a = stack.pop().unwrap();
                    while let Some(b) = stack.pop() {
                        swap_spaces(map, a, b);
                        a = b;
                    }
                } else {
                    let (next_left, next_right) = if map[next.0][next.1] == Space::LeftBox {
                        let next_left = next;
                        let next_right = (next.0, next.1 + 1);
                        assert_eq!(map[next_right.0][next_right.1], Space::RightBox);
                        (next_left, next_right)
                    } else {
                        let next_right = next;
                        let next_left = (next.0, next.1 - 1);
                        assert_eq!(map[next_left.0][next_left.1], Space::LeftBox);
                        (next_left, next_right)
                    };
                    // it's not actually a stack it's a HashSet for uniqueness, sue me
                    let mut stack = HashSet::new();
                    if move_boxes(map, &mut stack, 0, next_left, next_right, *dir) {
                        let mut stack: BinaryHeap<_> = stack.into_iter().collect();
                        while let Some(SpaceSwap { a, b, .. }) = stack.pop() {
                            swap_spaces(map, a, b);
                        }
                        swap_spaces(map, curr, next);
                    } else {
                        continue 'outer;
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
