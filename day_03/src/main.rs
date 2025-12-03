use std::cmp::{max, min};
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

fn solve(input: Vec<Vec<u8>>) -> (u64, u64) {
    let mut part_1: u64 = 0;
    let mut part_2: u64 = 0;

    for row in input {
        let mut digits_1: Vec<u8> = vec![row[0], 0];
        let mut index_1: usize = 0;
        let mut digits_2: Vec<u8> = vec![row[0], 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let mut index_2: usize = 0;

        for digit_index in 0..digits_2.len() {
            let mut max_1: usize = 0;
            if digit_index < 2 {
                max_1 = row.len() - (digits_1.len() - digit_index);
            } else {
                index_1 = 99;
            }

            let max_2 = row.len() - (digits_2.len() - digit_index);

            let start = min(index_1 + 1, index_2 + 1);
            let stop = max(max_1, max_2);

            for i in start..=stop {
                let n = row[i];
                if (i > index_1) && (i <= max_1)
                    && n > digits_1[digit_index] {
                        digits_1[digit_index] = n;
                        index_1 = i;
                    }
                if (i > index_2) && (i <= max_2)
                    && n > digits_2[digit_index] {
                        digits_2[digit_index] = n;
                        index_2 = i;
                    }
            }
        }

        let val_1: u64 = digits_1
            .iter()
            .fold(0, |acc, elem| acc * 10 + (*elem as u64));
        part_1 += val_1;

        let val_2: u64 = digits_2
            .iter()
            .fold(0, |acc, elem| acc * 10 + (*elem as u64));
        part_2 += val_2;
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
        assert_eq!(part_2, 3121910778619);
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
        assert_eq!(part_2, 171528556468625);
    }
}
