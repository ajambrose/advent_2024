use std::fs;

#[derive(Clone, Copy)]
struct Block {
    size: usize,
    file: Option<usize>,
}

impl Block {
    fn update(&mut self, rhs: Block) -> Option<Self> {
        assert!(self.size >= rhs.size);
        if self.size == rhs.size {
            *self = rhs;
            None
        } else {
            let ret = Block {
                size: self.size - rhs.size,
                file: None,
            };
            *self = rhs;
            Some(ret)
        }
    }
}

fn parse_file(s: &[char]) -> Vec<Block> {
    s.chunks(2)
        .enumerate()
        .map(|(i, c)| {
            let a = std::iter::once(Block {
                size: c[0].to_digit(10).unwrap() as usize,
                file: Some(i),
            });
            if let Some(b) = c.get(1) {
                a.chain(std::iter::once(Block {
                    size: b.to_digit(10).unwrap() as usize,
                    file: None,
                }))
            } else {
                a.chain(std::iter::once(Block {
                    size: 0,
                    file: None,
                }))
            }
        })
        .flatten()
        .collect()
}

fn solve(data: &mut Vec<Block>) -> usize {
    let mut i = 0usize;
    let mut j = data.len() - 1;
    'main: loop {
        while i < j {
            if data[i].file.is_none() {
                break;
            }
            i += 1;
        }
        while j > i {
            if data[j].file.is_some() {
                break;
            }
            j -= 1;
        }
        if i >= j {
            break;
        }
        for k in i..j {
            if data[k].file.is_none() && data[k].size >= data[j].size {
                let f = data[j];
                let new_block = data[k].update(f);
                data[j].file = None;
                if let Some(new_block) = new_block {
                    data.insert(k + 1, new_block);
                    j -= 1;
                }
                continue 'main;
            }
        }
        j -= 1;
    }

    data.iter()
        .map(|b| std::iter::repeat_n(b.file, b.size))
        .flatten()
        .enumerate()
        .filter_map(|(i, f)| f.map(|f| (i, f)))
        .fold(0usize, |acc, (i, f)| acc + (i * f))
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
