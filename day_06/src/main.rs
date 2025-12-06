use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::iter::zip;
use std::path::Path;

type ParsedInput = Vec<(char, Vec<u64>)>;

fn main() {
    let filename = env::args().nth(1).expect("Need filename");
    let input = read_input(filename);
    let (part_1, part_2) = solve(input);
    println!("{}\n{}", part_1, part_2);
}

fn read_input<P>(filename: P) -> ParsedInput
where
    P: AsRef<Path>,
{
    let mut numbers: Vec<Vec<u64>> = Vec::new();
    let mut operands: Vec<char> = Vec::new();

    let file = File::open(filename).unwrap();
    for line in io::BufReader::new(file).lines().map(|l| l.unwrap()) {
        let mut line_numbers: Vec<u64> = Vec::new();
        let parts = line.split(" ");
        for part in parts {
            if part.is_empty() {
                continue;
            }
            if part == "+" || part == "*" {
                operands.push(part.chars().next().unwrap());
            } else {
                line_numbers.push(part.parse::<u64>().unwrap())
            }
        }
        if !line_numbers.is_empty() {
            numbers.push(line_numbers);
        }
    }

    let mut input: Vec<(char, Vec<u64>)> = Vec::with_capacity(operands.len());
    for operand in operands.iter() {
        input.push((*operand, Vec::with_capacity(operands.len())));
    }

    for line_numbers in numbers.iter() {
        for (i, (_, number)) in zip(&operands, line_numbers).enumerate() {
            input[i].1.push(*number);
        }
    }

    input
}

fn solve(input: ParsedInput) -> (u64, u64) {
    let mut part_1: u64 = 0;
    let part_2: u64 = 0;

    for (operand, numbers) in input.iter() {
        if *operand == '+' {
            part_1 += numbers.iter().sum::<u64>();
        } else {
            part_1 += numbers.iter().product::<u64>();
        }
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
        assert_eq!(part_1, 4277556);
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
        assert_eq!(part_1, 4771265398012);
    }

    // this is for my personal puzzle input, yours will bit different
    #[test]
    fn test_part_2_real() {
        let input = read_input("./input.txt");
        let (_, part_2) = solve(input);
        assert_eq!(part_2, 0);
    }
}
