use std::cmp::{max, min};
use std::env;
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Copy, Clone, Debug)]
struct Range {
    start: u64,
    stop: u64,
}

impl Range {
    fn check(&self, item: u64) -> bool {
        self.start <= item && item <= self.stop
    }

    fn merge(&self, other: &Range) -> Option<Range> {
        if (self.start <= other.start && self.stop >= other.start)
            || (other.start <= self.start && other.stop >= self.stop)
        {
            Some(Range {
                start: min(self.start, other.start),
                stop: max(self.stop, other.stop),
            })
        } else {
            None
        }
    }
}

impl fmt::Display for Range {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "([{}-{}])", self.start, self.stop)
    }
}

type ParsedInput = (Vec<Range>, Vec<u64>);

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
    let mut fresh: Vec<Range> = Vec::new();
    let mut ingredients: Vec<u64> = Vec::new();
    let mut ranges: bool = true;

    let file = File::open(filename).unwrap();
    for line in io::BufReader::new(file).lines().map(|l| l.unwrap()) {
        if ranges {
            if line.is_empty() {
                ranges = false;
                continue;
            }

            let parts: Vec<&str> = line.split("-").collect();
            let start = parts[0].parse::<u64>().unwrap();
            let stop = parts[1].parse::<u64>().unwrap();
            fresh.push(Range { start, stop })
        } else {
            let item = line.parse::<u64>().unwrap();
            ingredients.push(item);
        }
    }

    (fresh, ingredients)
}

fn solve(input: ParsedInput) -> (u32, u32) {
    let mut part_1: u32 = 0;
    let part_2: u32 = 0;

    let (ranges, items) = input;
    let merged_ranges = merge_ranges(ranges);

    for item in items.iter() {
        for range in merged_ranges.iter() {
            if range.check(*item) {
                part_1 += 1;
                break;
            }
        }
    }

    (part_1, part_2)
}

fn merge_ranges(ranges: Vec<Range>) -> Vec<Range> {
    let mut merged_ranges = ranges.to_vec();
    merged_ranges.sort_by_key(|r| r.start);

    let mut index: usize = 0;
    while index < merged_ranges.len() - 1 {
        let mut range = merged_ranges[index];
        let mut merge_happened = true;

        while merge_happened {
            merge_happened = false;

            for j in index + 1..merged_ranges.len() {
                let merge = range.merge(&merged_ranges[j]);
                if let Some(merged) = merge {
                    range = merged;
                    merged_ranges[index] = merged;
                    merge_happened = true;
                    merged_ranges.remove(j);
                    break;
                }
            }
        }

        index += 1;
    }

    merged_ranges
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_example() {
        let input = read_input("./example.txt");
        let (part_1, _) = solve(input);
        assert_eq!(part_1, 3);
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
        assert_eq!(part_1, 770);
    }

    // this is for my personal puzzle input, yours will bit different
    #[test]
    fn test_part_2_real() {
        let input = read_input("./input.txt");
        let (_, part_2) = solve(input);
        assert_eq!(part_2, 0);
    }
}
