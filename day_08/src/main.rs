use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Box {
    x: u32,
    y: u32,
    z: u32,
}

impl Box {
    fn distance(&self, other: &Box) -> u64 {
        ((self.x as i64 - other.x as i64).pow(2)
            + (self.y as i64 - other.y as i64).pow(2)
            + (self.z as i64 - other.z as i64).pow(2)) as u64
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Connection {
    distance: u64,
    a: Box,
    b: Box,
}

#[derive(Clone)]
struct Circuit {
    boxes: HashSet<Box>,
    connections: HashSet<Connection>,
}

impl Circuit {
    fn add(&mut self, connection: Connection) {
        self.boxes.insert(connection.a);
        self.boxes.insert(connection.b);
        self.connections.insert(connection);
    }

    fn merge(&mut self, other: Circuit) {
        self.boxes.extend(other.boxes);
        self.connections.extend(other.connections);
    }

    fn size(&self) -> u64 {
        self.boxes.len() as u64
    }
}

struct Forest {
    circuits: Vec<Circuit>,
    box_map: HashMap<Box, usize>,
}

impl Forest {
    fn add(&mut self, connection: Connection) {
        let a_present = self.box_map.get(&connection.a).copied();
        let b_present = self.box_map.get(&connection.b).copied();

        match (a_present, b_present) {
            (None, None) => {
                // Neither box already in a circuit -> create new circuit
                let new_index = self.circuits.len();
                self.box_map.insert(connection.a, new_index);
                self.box_map.insert(connection.b, new_index);

                let mut new_circuit = Circuit {
                    boxes: HashSet::new(),
                    connections: HashSet::new(),
                };
                new_circuit.add(connection);
                self.circuits.push(new_circuit);
            }
            (Some(index), None) => {
                // box a already in a circuit -> add connection & extend circuit
                self.box_map.insert(connection.b, index);
                self.circuits[index].add(connection);
            }
            (None, Some(index)) => {
                // box b already in a circuit -> add connection & extend circuit
                self.box_map.insert(connection.a, index);
                self.circuits[index].add(connection);
            }
            (Some(index_a), Some(index_b)) => {
                // both boxes already in a circuit
                // if same circuit -> loop -> do nothing
                if index_a == index_b {
                    return;
                }

                // if separate circuits -> merge circuits
                let circuit_b = self.circuits[index_b].clone();
                for cbox in &circuit_b.boxes {
                    self.box_map.insert(*cbox, index_a);
                }

                self.circuits[index_a].merge(circuit_b);
            }
        }
    }

    fn largest(mut self) -> u64 {
        self.circuits
            .sort_unstable_by_key(|circuit| circuit.boxes.len());
        self.circuits.reverse();

        self.circuits[0].size() * self.circuits[1].size() * self.circuits[2].size()
    }
}

type ParsedInput = Vec<Box>;

fn main() {
    let filename = env::args().nth(1).expect("Need filename");
    let input = read_input(filename);
    let (part_1, part_2) = solve(input, 1000);
    println!("{}\n{}", part_1, part_2);
}

fn read_input<P>(filename: P) -> ParsedInput
where
    P: AsRef<Path>,
{
    let mut boxes: Vec<Box> = Vec::new();

    let file = File::open(filename).unwrap();
    for line in io::BufReader::new(file).lines().map(|l| l.unwrap()) {
        let parts: Vec<&str> = line.split(",").collect();
        boxes.push(Box {
            x: parts[0].parse::<u32>().unwrap(),
            y: parts[1].parse::<u32>().unwrap(),
            z: parts[2].parse::<u32>().unwrap(),
        });
    }

    boxes
}

fn solve(input: ParsedInput, n: usize) -> (u64, u64) {
    let connections = compute_connections(input);
    let mut forest = Forest {
        circuits: Vec::new(),
        box_map: HashMap::new(),
    };

    for connection in connections.iter().take(n) {
        forest.add(*connection)
    }

    (forest.largest(), 0)
}

fn compute_connections(boxes: Vec<Box>) -> Vec<Connection> {
    let n = boxes.len();
    let mut all_conns: Vec<Connection> = Vec::with_capacity((n * (n + 1)) / 2);
    for (i, box_a) in boxes.iter().enumerate() {
        for box_b in boxes[i + 1..].iter() {
            all_conns.push(Connection {
                a: *box_a,
                b: *box_b,
                distance: box_a.distance(box_b),
            })
        }
    }

    all_conns.sort();

    all_conns
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_example() {
        let input = read_input("./example.txt");
        let (part_1, _) = solve(input, 10);
        assert_eq!(part_1, 40);
    }

    #[test]
    fn test_part_2_example() {
        let input = read_input("./example.txt");
        let (_, part_2) = solve(input, 10);
        assert_eq!(part_2, 0);
    }

    // this is for my personal puzzle input, yours will bit different
    #[test]
    fn test_part_1_real() {
        let input = read_input("./input.txt");
        let (part_1, _) = solve(input, 1000);
        assert_eq!(part_1, 123234);
    }

    // this is for my personal puzzle input, yours will bit different
    #[test]
    fn test_part_2_real() {
        let input = read_input("./input.txt");
        let (_, part_2) = solve(input, 1000);
        assert_eq!(part_2, 0);
    }
}
