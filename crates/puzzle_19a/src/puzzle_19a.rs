use std::{collections::HashMap, fs};

fn parse_towels(s: &str) -> HashMap<String, bool> {
    s.split(", ").map(|s| (s.to_string(), true)).collect()
}

fn parse_file(s: &str) -> (HashMap<String, bool>, Vec<String>) {
    let mut ls = s.lines();
    let ts = ls.next().unwrap();
    let _ = ls.next().unwrap();
    let ds = ls.map(ToOwned::to_owned).collect();
    (parse_towels(ts), ds)
}

fn create_design(
    design: &str,
    towels: &HashMap<String, bool>,
    cache: &mut HashMap<String, bool>,
) -> bool {
    if design.is_empty() {
        return true;
    }

    if let Some(r) = cache.get(design) {
        return *r;
    }

    for (towel, _) in towels {
        if let Some(d) = design.strip_prefix(towel) {
            if create_design(d, towels, cache) {
                cache.insert(d.to_string(), true);
                return true;
            }
        }
    }

    cache.insert(design.to_string(), false);
    false
}

fn solve(towels: &HashMap<String, bool>, designs: &[String]) -> u64 {
    let mut total = 0;
    let mut cache = HashMap::new();
    for design in designs {
        if create_design(&design, towels, &mut cache) {
            total += 1;
        }
    }
    total
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let input = &args[1];
    let data = fs::read_to_string(input).unwrap();
    let (towels, designs) = parse_file(&data);
    println!("{}", solve(&towels, &designs));
}
