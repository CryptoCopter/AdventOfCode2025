use std::collections::{HashMap, HashSet, VecDeque};
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

type Height = u8;
type Column = u8;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Splitter {
    height: Height,
    column: Column,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Beam {
    start: Height,
    column: Column,
}

type ParsedInput = (Beam, HashMap<Column, Vec<Splitter>>);

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
    let mut start: Beam = Beam {
        start: 0,
        column: 0,
    };
    let mut height: Height = 0;
    let mut splitters: HashMap<Column, Vec<Splitter>> = HashMap::new();

    let file = File::open(filename).unwrap();
    for line in io::BufReader::new(file).lines().map(|l| l.unwrap()) {
        for (column, char) in line.chars().enumerate() {
            if char == 'S' {
                start.column = column as Column;
            }
            if char == '^' {
                let splitter = Splitter {
                    height: height as Height,
                    column: column as Column,
                };
                if let Some(splitters_column) = splitters.get_mut(&(column as Column)) {
                    splitters_column.push(splitter);
                } else {
                    let splitters_column: Vec<Splitter> = vec![splitter];
                    splitters.insert(column as Column, splitters_column);
                }
            }
        }

        height += 1;
    }

    (start, splitters)
}

fn solve(input: ParsedInput) -> (usize, u64) {
    let part_2: u64 = 0;
    let (start, mut splitters) = input;

    let mut beams: VecDeque<Beam> = VecDeque::new();
    beams.push_back(start);

    let mut hit_splitters: HashSet<Splitter> = HashSet::new();

    while !beams.is_empty() {
        let ray = beams.pop_front().unwrap();
        if let Some(column_splitters) = splitters.get_mut(&ray.column) {
            for splitter in column_splitters.iter() {
                if splitter.height > ray.start {
                    if hit_splitters.contains(splitter) {
                        break;
                    }

                    let ray_left = Beam {
                        start: splitter.height,
                        column: splitter.column - 1,
                    };
                    beams.push_back(ray_left);

                    let ray_right = Beam {
                        start: splitter.height,
                        column: splitter.column + 1,
                    };
                    beams.push_back(ray_right);

                    hit_splitters.insert(*splitter);

                    break;
                }
            }
        }
    }

    (hit_splitters.len(), part_2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_example() {
        let input = read_input("./example.txt");
        let (part_1, _) = solve(input);
        assert_eq!(part_1, 21);
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
        assert_eq!(part_1, 1698);
    }

    // this is for my personal puzzle input, yours will bit different
    #[test]
    fn test_part_2_real() {
        let input = read_input("./input.txt");
        let (_, part_2) = solve(input);
        assert_eq!(part_2, 0);
    }
}
