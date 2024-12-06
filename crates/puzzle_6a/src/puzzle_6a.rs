use std::fs;

#[derive(Clone, Copy)]
enum GuardDirection {
    Up,
    Right,
    Down,
    Left,
}

enum Space {
    Empty,
    Obstacle,
    Visited,
    Guard((GuardDirection, bool)),
}

fn turn_guard(dir: GuardDirection) -> GuardDirection {
    match dir {
        GuardDirection::Up => GuardDirection::Right,
        GuardDirection::Right => GuardDirection::Down,
        GuardDirection::Down => GuardDirection::Left,
        GuardDirection::Left => GuardDirection::Up,
    }
}

fn next_idx(dir: GuardDirection, i: usize, j: usize) -> Option<(usize, usize)> {
    match dir {
        GuardDirection::Up => Some((i.checked_sub(1)?, j)),
        GuardDirection::Right => Some((i, j + 1)),
        GuardDirection::Down => Some((i + 1, j)),
        GuardDirection::Left => Some((i, j.checked_sub(1)?)),
    }
}

fn move_guard(tiles: &mut Vec<Vec<Space>>, i: &mut usize, j: &mut usize) -> Option<bool> {
    let Space::Guard((dir, visited)) = tiles[*i][*j] else {
        panic!()
    };
    let (new_i, new_j) = next_idx(dir, *i, *j)?;
    match tiles.get(new_i)?.get(new_j)? {
        Space::Empty => {
            tiles[*i][*j] = Space::Visited;
            let next_tile = tiles.get_mut(new_i)?.get_mut(new_j)?;
            *next_tile = Space::Guard((dir, false));
            *i = new_i;
            *j = new_j;
            Some(!visited)
        }
        Space::Obstacle => {
            let Space::Guard((dir, _)) = &mut tiles[*i][*j] else {
                panic!()
            };
            *dir = turn_guard(*dir);
            Some(false)
        }
        Space::Visited => {
            tiles[*i][*j] = Space::Visited;
            let next_tile = tiles.get_mut(new_i)?.get_mut(new_j)?;
            *next_tile = Space::Guard((dir, true));
            *i = new_i;
            *j = new_j;
            Some(!visited)
        }
        Space::Guard(_) => unreachable!(),
    }
}

fn parse_file(file: &str) -> (Vec<Vec<Space>>, usize, usize) {
    let (mut i, mut j) = (0usize, 0usize);
    let tiles = file
        .lines()
        .enumerate()
        .map(|(line_idx, l)| {
            l.chars()
                .enumerate()
                .map(|(char_idx, c)| match c {
                    '.' => Space::Empty,
                    '#' => Space::Obstacle,
                    '^' => {
                        i = line_idx;
                        j = char_idx;
                        Space::Guard((GuardDirection::Up, false))
                    }
                    '>' => {
                        i = line_idx;
                        j = char_idx;
                        Space::Guard((GuardDirection::Right, false))
                    }
                    'v' => {
                        i = line_idx;
                        j = char_idx;
                        Space::Guard((GuardDirection::Down, false))
                    }
                    '<' => {
                        i = line_idx;
                        j = char_idx;
                        Space::Guard((GuardDirection::Left, false))
                    }
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();
    (tiles, i, j)
}

fn solve(tiles: &mut Vec<Vec<Space>>, mut i: usize, mut j: usize) -> u64 {
    let mut total = 0;
    while let Some(incr) = move_guard(tiles, &mut i, &mut j) {
        total += incr as u64;
    }
    if let Space::Visited = tiles[i][j] {
    } else {
        total += 1;
    }
    total
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let input = &args[1];
    let data = fs::read_to_string(input).unwrap();
    let (mut tiles, i, j) = parse_file(&data);
    println!("{}", solve(&mut tiles, i, j));
}
