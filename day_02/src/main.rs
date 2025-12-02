use fancy_regex::Regex;
use std::path::Path;
use std::{env, fs};

fn main() {
    let filename = env::args().nth(1).expect("Need filename");

    let input = read_input(filename);
    let (part_1, part_2) = solve(input);

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
        ))
    }

    ranges
}

fn solve(input: Vec<(u64, u64)>) -> (u64, u64) {
    let regex_1 = Regex::new(r"^(.*)\1$").unwrap();
    let mut sum: u64 = 0;

    for (start, stop) in input {
        for n in start..stop + 1 {
            let n_str = n.to_string();
            if regex_1.is_match(&n_str).unwrap() {
                sum += n;
            }
        }
    }

    (sum, 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_example() {
        let input = read_input("./example.txt");
        let (part_1, _) = solve(input);
        assert_eq!(part_1, 1227775554);
    }

    #[test]
    fn test_part_2_example() {
        let input = read_input("./example.txt");
        let (_, part_2) = solve(input);
        assert_eq!(part_2, 6);
    }

    #[test]
    fn test_part_1_real() {
        let input = read_input("./input.txt");
        let (part_1, _) = solve(input);
        assert_eq!(part_1, 1227775554);
    }

    #[test]
    fn test_part_2_real() {
        let input = read_input("./input.txt");
        let (_, part_2) = solve(input);
        assert_eq!(part_2, 6);
    }
}
