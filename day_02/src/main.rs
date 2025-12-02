use fancy_regex::Regex;
use std::path::Path;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::{env, fs};

fn main() {
    let filename = env::args().nth(1).expect("Need filename");
    let input = read_input(filename);
    let (part_1, part_2) = solve_parallel(input);
    println!("{}\n{}", part_1, part_2);
}

fn read_input<P>(filename: P) -> Vec<(u64, u64)>
where
    P: AsRef<Path>,
{
    let content = fs::read_to_string(filename).unwrap();
    let trimmed = content.trim();

    let mut ranges: Vec<(u64, u64)> = Vec::new();

    for range in trimmed.split(",") {
        let parts: Vec<&str> = range.split("-").collect();
        ranges.push((
            parts[0].parse::<u64>().unwrap(),
            parts[1].parse::<u64>().unwrap(),
        ));
    }

    ranges
}

fn solve(input: &[(u64, u64)], tx: Sender<(u64, u64)>) {
    let regex_1 = Regex::new(r"^(.*)\1$").unwrap();
    let regex_2 = Regex::new(r"^(.*)\1+$").unwrap();
    let mut sum_1: u64 = 0;
    let mut sum_2: u64 = 0;

    for (start, stop) in input {
        for n in *start..=*stop {
            let n_str = n.to_string();
            if regex_1.is_match(&n_str).unwrap() {
                sum_1 += n;
            }
            if regex_2.is_match(&n_str).unwrap() {
                sum_2 += n;
            }
        }
    }

    tx.send((sum_1, sum_2)).unwrap();
}

fn solve_parallel(input: Vec<(u64, u64)>) -> (u64, u64) {
    let mut sum_1: u64 = 0;
    let mut sum_2: u64 = 0;
    let cpus = num_cpus::get();
    let chunk_size = input.len().div_ceil(cpus);
    let chunks = input.chunks(chunk_size).map(|c| c.to_vec());
    let n_chunks = chunks.len();
    let (tx, rx): (Sender<(u64, u64)>, Receiver<(u64, u64)>) = mpsc::channel();

    for chunk in chunks {
        let txc = tx.clone();
        thread::spawn(move || {
            solve(&chunk, txc);
        });
    }

    for _ in 0..n_chunks {
        let (part_1, part_2) = rx.recv().unwrap();
        sum_1 += part_1;
        sum_2 += part_2;
    }

    (sum_1, sum_2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_example() {
        let input = read_input("./example.txt");
        let (part_1, _) = solve_parallel(input);
        assert_eq!(part_1, 1227775554);
    }

    #[test]
    fn test_part_2_example() {
        let input = read_input("./example.txt");
        let (_, part_2) = solve_parallel(input);
        assert_eq!(part_2, 4174379265);
    }

    // this is for my personal puzzle input, yours will bit different
    #[test]
    fn test_part_1_real() {
        let input = read_input("./input.txt");
        let (part_1, _) = solve_parallel(input);
        assert_eq!(part_1, 17077011375);
    }

    // this is for my personal puzzle input, yours will bit different
    #[test]
    fn test_part_2_real() {
        let input = read_input("./input.txt");
        let (_, part_2) = solve_parallel(input);
        assert_eq!(part_2, 36037497037);
    }
}
