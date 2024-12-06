use std::{
    collections::HashSet,
    fmt::{Formatter, Write},
    fs,
};

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
                        let mut s = HashSet::new();
                        s.insert(GuardDirection::Up);
                        Space::Guard((GuardDirection::Up, s))
                    }
                    '>' => {
                        i = line_idx;
                        j = char_idx;
                        let mut s = HashSet::new();
                        s.insert(GuardDirection::Right);
                        Space::Guard((GuardDirection::Right, s))
                    }
                    'v' => {
                        i = line_idx;
                        j = char_idx;
                        let mut s = HashSet::new();
                        s.insert(GuardDirection::Down);
                        Space::Guard((GuardDirection::Down, s))
                    }
                    '<' => {
                        i = line_idx;
                        j = char_idx;
                        let mut s = HashSet::new();
                        s.insert(GuardDirection::Left);
                        Space::Guard((GuardDirection::Left, s))
                    }
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();
    (tiles, i, j)
}

#[allow(unused)]
struct Tiles(Vec<Vec<Space>>);

impl std::fmt::Debug for Tiles {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        for row in &self.0 {
            for space in row {
                let c = match &space {
                    Space::Empty => '.',
                    Space::Obstacle => '#',
                    Space::Guard((dir, _)) => match dir {
                        GuardDirection::Up => '^',
                        GuardDirection::Right => '>',
                        GuardDirection::Down => 'v',
                        GuardDirection::Left => '<',
                    },
                    Space::Visited(_) => 'X',
                };
                fmt.write_char(c)?;
            }
            fmt.write_char('\n')?;
        }
        Ok(())
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum GuardDirection {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Clone)]
enum Space {
    Empty,
    Obstacle,
    Visited(HashSet<GuardDirection>),
    Guard((GuardDirection, HashSet<GuardDirection>)),
}

impl Default for Space {
    fn default() -> Self {
        Space::Empty
    }
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
    let Space::Guard((dir, mut prev)) = std::mem::take(&mut tiles[*i][*j]) else {
        panic!()
    };
    let (new_i, new_j) = next_idx(dir, *i, *j)?;
    match std::mem::take(tiles.get_mut(new_i)?.get_mut(new_j)?) {
        Space::Empty => {
            tiles[*i][*j] = Space::Visited(prev);
            let next_tile = tiles.get_mut(new_i)?.get_mut(new_j)?;
            let mut s = HashSet::new();
            s.insert(dir);
            *next_tile = Space::Guard((dir, s));
            *i = new_i;
            *j = new_j;
            Some(false)
        }
        Space::Obstacle => {
            tiles[new_i][new_j] = Space::Obstacle;
            let new_dir = turn_guard(dir);
            if !prev.insert(new_dir) {
                tiles[*i][*j] = Space::Guard((new_dir, prev));
                return Some(true);
            }
            tiles[*i][*j] = Space::Guard((new_dir, prev));
            Some(false)
        }
        Space::Visited(mut visited_dirs) => {
            if !visited_dirs.insert(dir) {
                tiles[*i][*j] = Space::Guard((dir, prev));
                return Some(true);
            }
            tiles[*i][*j] = Space::Visited(prev);
            let next_tile = tiles.get_mut(new_i)?.get_mut(new_j)?;
            *next_tile = Space::Guard((dir, visited_dirs));
            *i = new_i;
            *j = new_j;
            Some(false)
        }
        Space::Guard(_) => unreachable!(),
    }
}

fn solve(tiles: &mut Vec<Vec<Space>>, mut i: usize, mut j: usize) -> bool {
    while let Some(incr) = move_guard(tiles, &mut i, &mut j) {
        if incr {
            return true;
        }
    }
    return false;
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let input = &args[1];
    let data = fs::read_to_string(input).unwrap();
    let (tiles, i, j) = parse_file(&data);
    // lol how bad could this be
    let mut total = 0;
    for oi in 0..tiles.len() {
        for oj in 0..tiles[oi].len() {
            if let Space::Empty = &tiles[oi][oj] {
                let mut new_tiles = tiles.clone();
                new_tiles[oi][oj] = Space::Obstacle;
                total += solve(&mut new_tiles, i, j) as u64;
            }
        }
    }
    println!("{}", total);
}
