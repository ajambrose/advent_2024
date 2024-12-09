use std::fs;

type Block = Option<usize>;

fn parse_file(s: &[char]) -> Vec<Block> {
    s.chunks(2)
        .enumerate()
        .map(|(i, c)| {
            let a = std::iter::repeat_n(Some(i), c[0].to_digit(10).unwrap() as usize);
            if c.len() == 2 {
                a.chain(std::iter::repeat_n(
                    None,
                    c[1].to_digit(10).unwrap() as usize,
                ))
            } else {
                a.chain(std::iter::repeat_n(None, 0))
            }
        })
        .flatten()
        .collect()
}

fn solve(data: &mut [Block]) -> usize {
    let mut i = 0usize;
    let mut j = data.len() - 1;
    loop {
        while i < j {
            if data[i].is_none() {
                break;
            }
            i += 1;
        }
        while j > i {
            if data[j].is_some() {
                break;
            }
            j -= 1;
        }
        if i >= j {
            break;
        }
        data.swap(i, j);
    }

    data.iter()
        .map_while(|&b| b)
        .enumerate()
        .fold(0usize, |acc, (i, b)| acc + (i * b))
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let input = &args[1];
    let data: Vec<_> = fs::read_to_string(input)
        .unwrap()
        .chars()
        .take_while(|c| /* thanks windows */ *c != '\r' && *c != '\n')
        .collect();
    let mut v = parse_file(&data);
    println!("{}", solve(&mut v));
}
