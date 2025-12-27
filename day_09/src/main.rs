use std::cmp::max;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Tile {
    row: isize,
    column: isize,
}

fn read_input<P>(filename: P) -> Vec<Tile>
where
    P: AsRef<Path>,
{
    let mut tiles: Vec<Tile> = Vec::new();

    let file = File::open(filename).unwrap();
    for line in io::BufReader::new(file).lines().map(|l| l.unwrap()) {
        let parts: Vec<&str> = line.split(",").collect();
        let column = parts[0].parse::<isize>().unwrap();
        let row = parts[1].parse::<isize>().unwrap();
        tiles.push(Tile { row, column });
    }

    tiles.sort_unstable();

    tiles
}

fn solve(input: Vec<Tile>) -> (u64, u64) {
    let mut part_1: u64 = 0;

    for (index, tile) in input.iter().enumerate() {
        for other_tile in input[index + 1..].iter() {
            let row_diff = (tile.row - other_tile.row).unsigned_abs() as u64;
            let column_dif = (tile.column - other_tile.column).unsigned_abs() as u64;
            let area = (row_diff + 1) * (column_dif + 1);
            part_1 = max(part_1, area);
        }
    }

    (part_1, 0)
}

fn main() {
    let filename = env::args().nth(1).expect("Need filename");
    let input = read_input(filename);
    let (part_1, part_2) = solve(input);
    println!("{}\n{}", part_1, part_2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_example() {
        let input = read_input("./example.txt");
        let (part_1, _) = solve(input);
        assert_eq!(part_1, 50);
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
        assert_eq!(part_1, 0);
    }

    // this is for my personal puzzle input, yours will bit different
    #[test]
    fn test_part_2_real() {
        let input = read_input("./input.txt");
        let (_, part_2) = solve(input);
        assert_eq!(part_2, 0);
    }
}
