use std::fs;

#[derive(Debug)]
#[repr(u8)]
enum Instruction {
    Adv = 0,
    Bxl = 1,
    Bst = 2,
    Jnz = 3,
    Bxc = 4,
    Out = 5,
    Bdv = 6,
    Cdv = 7,
}

impl TryFrom<u8> for Instruction {
    type Error = ();
    fn try_from(i: u8) -> Result<Self, <Self as TryFrom<u8>>::Error> {
        Ok(match i {
            0 => Self::Adv,
            1 => Self::Bxl,
            2 => Self::Bst,
            3 => Self::Jnz,
            4 => Self::Bxc,
            5 => Self::Out,
            6 => Self::Bdv,
            7 => Self::Cdv,
            _ => return Err(()),
        })
    }
}

fn parse_file(s: &str) -> (u64, u64, u64, Vec<u8>) {
    let mut ls = s.lines();
    let a = ls.next().unwrap();
    let b = ls.next().unwrap();
    let c = ls.next().unwrap();
    let _ = ls.next().unwrap();
    let i = ls.next().unwrap();

    let a: u64 = a.strip_prefix("Register A: ").unwrap().parse().unwrap();
    let b: u64 = b.strip_prefix("Register B: ").unwrap().parse().unwrap();
    let c: u64 = c.strip_prefix("Register C: ").unwrap().parse().unwrap();
    let v: Vec<u8> = i
        .strip_prefix("Program: ")
        .unwrap()
        .split(',')
        .map(|i| i.parse().unwrap())
        .collect();
    (a, b, c, v)
}

fn read_combo(a: u64, b: u64, c: u64, op: u8) -> u64 {
    return match op {
        0 | 1 | 2 | 3 => op as u64,
        4 => a,
        5 => b,
        6 => c,
        _ => unreachable!(),
    } & 0x7;
}

fn solve(mut a: u64, mut b: u64, mut c: u64, p: &[u8], output: &mut String) -> Option<()> {
    let mut ip = 0usize;
    println!("Initial: {a} {b} {c} {p:?}");

    loop {
        let i = (*p.get(ip)?).try_into().unwrap();
        // println!("i: {i:?}");
        match i {
            Instruction::Adv => {
                a = a / (1u64.checked_shl(read_combo(a, b, c, *p.get(ip + 1)?) as u32).unwrap());
            }
            Instruction::Bxl => {
                b = b ^ *p.get(ip + 1)? as u64;
            }
            Instruction::Bst => {
                b = read_combo(a, b, c, *p.get(ip + 1)?) & 0x7;
            }
            Instruction::Jnz => {
                if a != 0 {
                    ip = *p.get(ip + 1)? as usize;
                    continue;
                }
            }
            Instruction::Bxc => {
                b ^= c;
            }
            Instruction::Out => {
                output.push_str(&(read_combo(a, b, c, *p.get(ip + 1)?) & 0x7).to_string());
                output.push(',');
            }
            Instruction::Bdv => {
                b = a / (1u64.checked_shl(read_combo(a, b, c, *p.get(ip + 1)?) as u32).unwrap());
            }
            Instruction::Cdv => {
                c = a / (1u64.checked_shl(read_combo(a, b, c, *p.get(ip + 1)?) as u32).unwrap());
            }
        }
        ip += 2;
    }
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let input = &args[1];
    let data = fs::read_to_string(input).unwrap();
    let (a, b, c, p) = parse_file(&data);
    let mut output = String::default();
    solve(a, b, c, &p, &mut output);
    println!("{output}");
    // let output: String = output.chars().filter(|&c| c != ',').collect();
    // println!("{output}");
}
