use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

type Present = u32;

struct Area {
    x: u32,
    y: u32,
    presents: [Present; 6],
}

fn read_input<P>(filename: P) -> (Vec<Area>, [Present; 6])
where
    P: AsRef<Path>,
{
    let mut presents: [Present; 6] = [0, 0, 0, 0, 0, 0];
    let mut areas: Vec<Area> = Vec::new();
    let mut parsing_presents = true;
    let mut present_index: usize = 0;

    let file = File::open(filename).unwrap();
    for line in io::BufReader::new(file).lines().map(|l| l.unwrap()) {
        if parsing_presents {
            if present_index == 5 && line.is_empty() {
                parsing_presents = false;
                continue;
            }
            if line.is_empty() {
                present_index += 1;
                continue;
            }

            let present_area: u32 = line
                .chars()
                .collect::<Vec<char>>()
                .iter()
                .filter(|c| **c == '#')
                .copied()
                .collect::<Vec<char>>()
                .len() as u32;
            presents[present_index] += present_area;
        } else {
            let parts: Vec<&str> = line.split(": ").collect();
            let size: Vec<&str> = parts[0].split("x").collect();
            let area_presents: Vec<&str> = parts[1].split(" ").collect();
            let area = Area {
                x: size[0].parse::<u32>().unwrap(),
                y: size[1].parse::<u32>().unwrap(),
                presents: [
                    area_presents[0].parse::<u32>().unwrap(),
                    area_presents[1].parse::<u32>().unwrap(),
                    area_presents[2].parse::<u32>().unwrap(),
                    area_presents[3].parse::<u32>().unwrap(),
                    area_presents[4].parse::<u32>().unwrap(),
                    area_presents[5].parse::<u32>().unwrap(),
                ],
            };
            areas.push(area);
        }
    }

    (areas, presents)
}

fn solve(input: (Vec<Area>, [Present; 6])) -> (u64, u64) {
    let (areas, presents) = input;
    let mut part_1: u64 = 0;

    for area in areas {
        let required: u32 = presents[0] * area.presents[0]
            + presents[1] * area.presents[1]
            + presents[2] * area.presents[2]
            + presents[3] * area.presents[3]
            + presents[4] * area.presents[4]
            + presents[5] * area.presents[5];
        if required < area.x * area.y {
            part_1 += 1;
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
        assert_eq!(part_1, 2);
    }

    #[test]
    fn test_part_2_example() {
        let input = read_input("./example.txt");
        let (_, part_2) = solve(input);
        assert_eq!(part_2, 538);
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
