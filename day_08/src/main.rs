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

impl Connection {
    fn distance_wall(&self) -> u64 {
        self.a.x as u64 * self.b.x as u64
    }
}

#[derive(Clone)]
struct Circuit {
    index: usize,
    boxes: HashSet<Box>,
}

impl Circuit {
    fn add(&mut self, connection: Connection) {
        self.boxes.insert(connection.a);
        self.boxes.insert(connection.b);
    }

    fn merge(&mut self, other: Circuit) {
        self.boxes.extend(other.boxes);
    }

    fn size(&self) -> u64 {
        self.boxes.len() as u64
    }
}

struct Forest {
    circuits: Vec<Circuit>,
    box_map: HashMap<Box, usize>,
    active_circuits: HashSet<usize>,
}

impl Forest {
    fn from_boxes(boxes: &[Box]) -> Forest {
        let mut forest = Forest {
            circuits: Vec::with_capacity(boxes.len()),
            box_map: HashMap::with_capacity(boxes.len()),
            active_circuits: HashSet::with_capacity(boxes.len()),
        };

        for (index, cbox) in boxes.iter().enumerate() {
            let circuit = Circuit {
                index,
                boxes: HashSet::from([*cbox]),
            };
            forest.circuits.push(circuit);
            forest.box_map.insert(*cbox, index);
            forest.active_circuits.insert(index);
        }

        forest
    }

    fn add(&mut self, connection: Connection) {
        let a_present = self.box_map.get(&connection.a).copied();
        let b_present = self.box_map.get(&connection.b).copied();

        match (a_present, b_present) {
            (None, None) => (),
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
                self.active_circuits.remove(&index_b);
            }
        }
    }

    fn largest(&self) -> u64 {
        let mut current_circuits = self.circuits.clone();
        current_circuits.retain(|circ| self.active_circuits.contains(&circ.index));

        current_circuits.sort_unstable_by_key(|circuit| circuit.boxes.len());
        current_circuits.reverse();

        current_circuits[0].size() * current_circuits[1].size() * current_circuits[2].size()
    }

    fn dense(&self) -> bool {
        self.active_circuits.len() == 1
    }
}

fn main() {
    let filename = env::args().nth(1).expect("Need filename");
    let input = read_input(filename);
    let (part_1, part_2) = solve(input, 1000);
    println!("{}\n{}", part_1, part_2);
}

fn read_input<P>(filename: P) -> Vec<Box>
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

fn solve(input: Vec<Box>, n: usize) -> (u64, u64) {
    let mut forest = Forest::from_boxes(&input);
    let connections = compute_connections(input);

    // part 1
    for connection in connections.iter().take(n) {
        forest.add(*connection)
    }
    let part_1 = forest.largest();

    // continue with part 2
    let mut index = n;
    while !forest.dense() {
        forest.add(connections[index]);
        index += 1;
    }
    let last_connection = connections[index - 1];

    (part_1, last_connection.distance_wall())
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
        assert_eq!(part_2, 25272);
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
        assert_eq!(part_2, 9259958565);
    }
}
