use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;

fn main() {
    let filename = env::args().nth(1).expect("Need filename");

    if let Ok(input) = read_input(filename) {
        let (part_1, part_2) = solve(input);

        println!("{}\n{}", part_1, part_2);
    }
}

fn read_input<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn solve(input: io::Lines<io::BufReader<File>>) -> (u16, u16) {
    let mut dial: i16 = 50;
    let mut hits: u16 = 0;
    let mut driveby: u16 = 0;

    for line in input.map_while(Result::ok) {
        let direction = line.as_bytes()[0];
        let mut steps = line[1..].parse::<i16>().unwrap();
        let mut sweeps: u16 = 0;

        if direction == 76 {
            steps *= -1;
        }

        let old_dial = dial;
        dial = dial + steps;
        if dial > 99 {
            sweeps = (dial / 100) as u16;
        }
        if dial < 1 {
            sweeps = (dial / 100).abs() as u16;
            if old_dial != 0 {
                sweeps += 1;
            }
        }
        driveby += sweeps;

        dial = dial % 100;

        if dial < 0 {
            dial = dial + 100;
        }

        if dial == 0 {
            hits += 1;
        }
    }
    (hits, driveby)
}
