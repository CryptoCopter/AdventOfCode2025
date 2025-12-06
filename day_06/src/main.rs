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

fn read_input<P>(filename: P) -> (ParsedInput, ParsedInput)
where
    P: AsRef<Path>,
{
    let mut numbers: Vec<Vec<u64>> = Vec::new();
    let mut operands: Vec<char> = Vec::new();
    let mut chars: Vec<Vec<char>> = Vec::new();

    let file = File::open(filename).unwrap();
    for line in io::BufReader::new(file).lines().map(|l| l.unwrap()) {
        chars.push(line.chars().collect());
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

    let input_1 = zip_input(operands, numbers);
    let input_2 = transform_input(chars);

    (input_1, input_2)
}

fn transform_input(mut input: Vec<Vec<char>>) -> ParsedInput {
    let max_len: usize = input.iter().map(|l| l.len()).max().unwrap();
    for row in input.iter_mut() {
        while row.len() < max_len {
            row.push(' ');
        }
    }

    let mut transposed: Vec<Vec<char>> = transpose_ragged(input);
    let mut operands: Vec<char> = Vec::new();

    let mut numbers: Vec<Vec<u64>> = Vec::new();
    let mut line_numbers: Vec<u64> = Vec::new();
    for column in transposed.iter_mut().rev() {
        if let Some(&last) = column.last()
            && (last == '+' || last == '*')
        {
            operands.push(last);
            column.pop();
        }
        let num_str: String = column.iter().collect();
        let trimmed = num_str.trim();
        if trimmed.is_empty() {
            numbers.push(line_numbers);
            line_numbers = Vec::new();
            continue;
        }

        line_numbers.push(trimmed.parse::<u64>().unwrap())
    }
    numbers.push(line_numbers);

    let transformed: ParsedInput = zip(operands, numbers).collect();

    transformed
}

fn zip_input(operands: Vec<char>, numbers: Vec<Vec<u64>>) -> ParsedInput {
    let mut zipped: ParsedInput = Vec::with_capacity(operands.len());
    for operand in operands.iter() {
        zipped.push((*operand, Vec::with_capacity(operands.len())));
    }

    for line_numbers in numbers.iter() {
        for (i, (_, number)) in zip(&operands, line_numbers).enumerate() {
            zipped[i].1.push(*number);
        }
    }

    zipped
}

fn transpose_ragged<T: Copy>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let rows = v.len();
    let cols = v.iter().map(|row| row.len()).min().unwrap_or(0);

    (0..cols)
        .map(|c| (0..rows).map(|r| v[r][c]).collect())
        .collect()
}

fn solve(input: (ParsedInput, ParsedInput)) -> (u64, u64) {
    let part_1 = compute_solution(input.0);
    let part_2 = compute_solution(input.1);

    (part_1, part_2)
}

fn compute_solution(input: ParsedInput) -> u64 {
    let mut sum: u64 = 0;

    for (operand, numbers) in input.iter() {
        if *operand == '+' {
            sum += numbers.iter().sum::<u64>();
        } else {
            sum += numbers.iter().product::<u64>();
        }
    }

    sum
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
        assert_eq!(part_2, 3263827);
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
        assert_eq!(part_2, 10695785245101);
    }
}
