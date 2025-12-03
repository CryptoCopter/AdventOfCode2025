use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let filename = env::args().nth(1).expect("Need filename");
    let input = read_input(filename);
    let (part_1, part_2) = solve(input);
    println!("{}\n{}", part_1, part_2);
}

fn read_input<P>(filename: P) -> Vec<Vec<u8>>
where
    P: AsRef<Path>,
{
    let mut ranges: Vec<Vec<u8>> = Vec::new();

    let file = File::open(filename).unwrap();
    for line in io::BufReader::new(file).lines().map(|l| l.unwrap()) {
        let mut digits: Vec<u8> = Vec::new();
        for char in line.chars() {
            let digit = char.to_digit(10).unwrap() as u8;
            digits.push(digit);
        }
        ranges.push(digits);
    }

    ranges
}

fn solve(input: Vec<Vec<u8>>) -> (u32, u32) {
    let mut part_1: u32 = 0;
    let part_2: u32 = 0;

    for row in input {
        let mut ten: u8 = row[0];
        let mut ten_index: usize = 0;
        let mut one: u8 = 0;

        for (i, digit) in row[..row.len() - 1].iter().enumerate() {
            if *digit > ten {
                ten = *digit;
                ten_index = i;
            }
        }

        for digit in row[ten_index + 1..].iter() {
            if *digit > one {
                one = *digit;
            }
        }

        let joltage: u8 = (ten * 10) + one;
        part_1 += joltage as u32;
    }

    (part_1, part_2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_example() {
        let input = read_input("./example.txt");
        let (part_1, _) = solve(input);
        assert_eq!(part_1, 357);
    }

    #[test]
    fn test_part_2_example() {
        let input = read_input("./example.txt");
        let (_, part_2) = solve(input);
        assert_eq!(part_2, 0);
    }

    // this is for my personal puzzle input, yours will bit different
    #[test]
    fn test_part_1_real() {
        let input = read_input("./input.txt");
        let (part_1, _) = solve(input);
        assert_eq!(part_1, 17278);
    }

    // this is for my personal puzzle input, yours will bit different
    #[test]
    fn test_part_2_real() {
        let input = read_input("./input.txt");
        let (_, part_2) = solve(input);
        assert_eq!(part_2, 0);
    }
}
