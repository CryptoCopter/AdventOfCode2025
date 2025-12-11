use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct Device {
    name: String,
    outputs: Vec<String>,
}

struct Graph {
    root: usize,
    svr: usize,
    fft: usize,
    dac: usize,
    devices: Vec<Device>,
    name_map: HashMap<String, usize>,
}

impl Graph {
    fn new() -> Graph {
        let mut graph = Graph {
            root: 0,
            svr: 0,
            fft: 0,
            dac: 0,
            devices: vec![Device {
                name: "out".to_string(),
                outputs: Vec::new(),
            }],
            name_map: HashMap::new(),
        };

        graph.name_map.insert("out".to_string(), 0);

        graph
    }

    fn add(&mut self, device: Device) {
        let index = self.devices.len();
        let name: String = device.name.clone();

        if name == "you" {
            self.root = index;
        }
        if name == "svr" {
            self.svr = index;
        }
        if name == "fft" {
            self.fft = index;
        }
        if name == "dac" {
            self.dac = index;
        }

        self.name_map.insert(name, index);
        self.devices.push(device);
    }

    fn walk(&self) -> u64 {
        let mut cache: HashMap<String, u64> = HashMap::with_capacity(self.devices.len());
        self._walk(&self.devices[self.root], "out", &mut cache)
    }

    fn _walk(&self, device: &Device, goal: &str, cache: &mut HashMap<String, u64>) -> u64 {
        if let Some(cached) = cache.get(&device.name) {
            return *cached;
        }

        if device.name == goal {
            return 1;
        }

        let mut sum: u64 = 0;

        for output in &device.outputs {
            sum += self._walk(
                &self.devices[*self.name_map.get(output).unwrap()],
                goal,
                cache,
            )
        }

        cache.insert(device.name.clone(), sum);
        sum
    }

    fn paths_through(&self) -> u64 {
        let mut cache: HashMap<String, u64> = HashMap::with_capacity(self.devices.len());
        let paths_fft = self._walk(&self.devices[self.svr], "fft", &mut cache);

        cache = HashMap::with_capacity(self.devices.len());
        let paths_dac = self._walk(&self.devices[self.fft], "dac", &mut cache);

        cache = HashMap::with_capacity(self.devices.len());
        let paths_out = self._walk(&self.devices[self.dac], "out", &mut cache);

        paths_fft * paths_dac * paths_out
    }
}

fn read_input<P>(filename: P) -> Graph
where
    P: AsRef<Path>,
{
    let mut graph = Graph::new();

    let file = File::open(filename).unwrap();
    for line in io::BufReader::new(file).lines().map(|l| l.unwrap()) {
        let parts: Vec<&str> = line.split(": ").collect();
        let name: String = parts[0].to_string();
        let outputs: Vec<String> = parts[1].split(" ").map(|out| out.to_string()).collect();

        let device = Device { name, outputs };

        graph.add(device);
    }

    graph
}

fn solve(graph: Graph) -> (u64, u64) {
    (graph.walk(), graph.paths_through())
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
        assert_eq!(part_1, 5);
    }

    #[test]
    fn test_part_2_example() {
        let input = read_input("./example_2.txt");
        let (_, part_2) = solve(input);
        assert_eq!(part_2, 2);
    }

    // this is for my personal puzzle input, yours will bit different
    #[test]
    fn test_part_1_real() {
        let input = read_input("./input.txt");
        let (part_1, _) = solve(input);
        assert_eq!(part_1, 543);
    }

    // this is for my personal puzzle input, yours will bit different
    #[test]
    fn test_part_2_real() {
        let input = read_input("./input.txt");
        let (_, part_2) = solve(input);
        assert_eq!(part_2, 479511112939968);
    }
}
