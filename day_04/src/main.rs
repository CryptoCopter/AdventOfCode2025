use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::ops;
use std::path::Path;

#[derive(Copy, Clone, Debug)]
struct Location {
    row: usize,
    column: usize,
}

impl ops::Add<Direction> for Location {
    type Output = Result<Location, &'static str>;

    fn add(self, _rhs: Direction) -> Result<Location, &'static str> {
        let irow: isize = (self.row as isize) + _rhs.d_row;
        let icolumn: isize = (self.column as isize) + _rhs.d_column;
        if irow < 0 || icolumn < 0 {
            return Err("Indices can't be < 0");
        }

        Ok(Location {
            row: irow as usize,
            column: icolumn as usize,
        })
    }
}

impl ops::Add<&Direction> for &Location {
    type Output = Result<Location, &'static str>;

    fn add(self, _rhs: &Direction) -> Result<Location, &'static str> {
        let irow: isize = (self.row as isize) + _rhs.d_row;
        let icolumn: isize = (self.column as isize) + _rhs.d_column;
        if irow < 0 || icolumn < 0 {
            return Err("Indices can't be < 0");
        }

        Ok(Location {
            row: irow as usize,
            column: icolumn as usize,
        })
    }
}

#[derive(Copy, Clone, Debug)]
struct Direction {
    d_row: isize,
    d_column: isize,
}

impl ops::Add<Direction> for Direction {
    type Output = Direction;

    fn add(self, _rhs: Direction) -> Direction {
        Direction {
            d_row: self.d_row + _rhs.d_row,
            d_column: self.d_column + _rhs.d_column,
        }
    }
}

impl ops::Mul<isize> for Direction {
    type Output = Direction;

    fn mul(self, _rhs: isize) -> Direction {
        Direction {
            d_row: self.d_row * _rhs,
            d_column: self.d_column * _rhs,
        }
    }
}

const LEFT: Direction = Direction {
    d_row: 0,
    d_column: -1,
};
const RIGHT: Direction = Direction {
    d_row: 0,
    d_column: 1,
};
const UP: Direction = Direction {
    d_row: 1,
    d_column: 0,
};
const DOWN: Direction = Direction {
    d_row: -1,
    d_column: 0,
};

struct FloorPlan {
    area: Vec<Vec<bool>>,
}

impl FloorPlan {
    fn check(&self, location: &Location) -> Result<bool, &'static str> {
        if location.row >= self.area.len() {
            return Err("Out of bounds");
        }
        let row = &self.area[location.row];
        if location.column >= row.len() {
            return Err("Out of bounds");
        }
        Ok(row[location.column])
    }

    fn remove(&mut self, location: &Location) -> Result<(), &'static str> {
        if location.row >= self.area.len() {
            return Err("Out of bounds");
        }
        if location.column >= self.area[location.row].len() {
            return Err("Out of bounds");
        }
        self.area[location.row][location.column] = false;
        Ok(())
    }
}

type ParsedInput = (FloorPlan, Vec<Location>);

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
    let mut floorplan: Vec<Vec<bool>> = Vec::new();
    let mut rolls: Vec<Location> = Vec::new();

    let file = File::open(filename).unwrap();
    for (row_index, line) in io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap())
        .enumerate()
    {
        let mut row: Vec<bool> = Vec::new();
        for (column_index, char) in line.chars().enumerate() {
            match char {
                '@' => {
                    row.push(true);
                    rolls.push(Location {
                        row: row_index,
                        column: column_index,
                    });
                }
                _ => row.push(false),
            }
        }
        floorplan.push(row);
    }

    (FloorPlan { area: floorplan }, rolls)
}

fn solve(input: ParsedInput) -> (u32, u32) {
    let mut part_1: u32 = 0;
    let mut done_1: bool = false;
    let mut part_2: u32 = 0;
    let (mut floorplan, mut rolls) = input;
    let around: [Direction; 8] = [
        LEFT,
        LEFT + UP,
        UP,
        UP + RIGHT,
        RIGHT,
        RIGHT + DOWN,
        DOWN,
        DOWN + LEFT,
    ];

    loop {
        let mut removable: Vec<usize> = Vec::new();
        for (roll_index, roll) in rolls.iter().enumerate() {
            let mut neighbours: u8 = 0;
            for direction in around.iter() {
                let step = roll + direction;
                if let Ok(location) = step {
                    let check = floorplan.check(&location);
                    if let Ok(neighbour) = check
                        && neighbour
                    {
                        neighbours += 1;
                    }
                }
            }
            if neighbours < 4 {
                removable.push(roll_index);
            }
        }

        let n_remove = removable.len() as u32;

        if n_remove == 0 {
            break;
        }

        if !done_1 {
            part_1 += n_remove;
            done_1 = true;
        }
        part_2 += n_remove;

        for roll_index in removable.iter().rev() {
            floorplan.remove(&rolls[*roll_index]).unwrap();
            rolls.remove(*roll_index);
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
        assert_eq!(part_1, 13);
    }

    #[test]
    fn test_part_2_example() {
        let input = read_input("./example.txt");
        let (_, part_2) = solve(input);
        assert_eq!(part_2, 43);
    }

    // this is for my personal puzzle input, yours will bit different
    #[test]
    fn test_part_1_real() {
        let input = read_input("./input.txt");
        let (part_1, _) = solve(input);
        assert_eq!(part_1, 1416);
    }

    // this is for my personal puzzle input, yours will bit different
    #[test]
    fn test_part_2_real() {
        let input = read_input("./input.txt");
        let (_, part_2) = solve(input);
        assert_eq!(part_2, 9086);
    }
}
