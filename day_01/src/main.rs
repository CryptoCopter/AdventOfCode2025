use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(input) = read_input("./input.txt") {
        let mut dial: i16 = 50;
        let mut hits: u16 = 0;

        for line in input.map_while(Result::ok) {
            let direction = line.as_bytes()[0];
            let mut steps = line[1..].parse::<i16>().unwrap();

            if direction == 76 {
                steps *= -1;
            }

            dial = (dial + steps) % 100;

            if dial < 0 {
                dial = dial + 100;
            }

            if dial == 0 {
                hits += 1;
            }
        }

        println!("{}", hits);
    }
}

fn read_input<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
