use std::collections::{HashMap, VecDeque};
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

type Height = u8;
type Column = u8;

#[derive(Debug)]
enum Side {
    Left,
    Right,
}

#[derive(Debug)]
struct Beam {
    start: Height,
    column: Column,
    origin: Option<Splitter>,
    side: Option<Side>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Splitter {
    height: Height,
    column: Column,
}

struct SplitterTree {
    root: Option<Splitter>,
    indices: HashMap<Splitter, usize>,
    nodes: Vec<TreeNode>,
}

impl SplitterTree {
    fn insert(&mut self, node: TreeNode) -> Result<(), &'static str> {
        if self.indices.contains_key(&node.splitter) {
            return Err("Node already in tree");
        }

        if self.root.is_none() {
            self.root = Some(node.splitter)
        }

        let node_index = self.nodes.len();

        self.indices.insert(node.splitter, node_index);
        self.nodes.push(node);

        Ok(())
    }

    fn get(&self, splitter: &Splitter) -> Result<&TreeNode, &'static str> {
        if let Some(node_index) = self.indices.get(splitter) {
            Ok(&self.nodes[*node_index])
        } else {
            Err("Splitter not in Tree")
        }
    }

    fn contains(&self, splitter: &Splitter) -> bool {
        self.indices.contains_key(splitter)
    }

    fn connect(
        &mut self,
        parent: Splitter,
        child: Splitter,
        side: Side,
    ) -> Result<(), &'static str> {
        if let Some(parent_index) = self.indices.get(&parent) {
            let parent_node = &mut self.nodes[*parent_index];

            match side {
                Side::Left => {
                    if parent_node.left_child.is_some() {
                        return Err("Child already occupied");
                    }
                    parent_node.left_child = Some(child);
                }
                Side::Right => {
                    if parent_node.right_child.is_some() {
                        return Err("Child already occupied");
                    }
                    parent_node.right_child = Some(child);
                }
            }
        } else {
            return Err("Parent not in tree");
        }

        Ok(())
    }

    fn size(&self) -> usize {
        self.indices.len()
    }
}

struct TreeNode {
    splitter: Splitter,
    left_child: Option<Splitter>,
    right_child: Option<Splitter>,
}

type ParsedInput = (Column, HashMap<Column, Vec<Splitter>>);

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
    let mut start: Column = 0;
    let mut splitter_map: HashMap<Column, Vec<Splitter>> = HashMap::new();

    let file = File::open(filename).unwrap();
    for (height, line) in io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap())
        .enumerate()
    {
        for (column, char) in line.chars().enumerate() {
            if char == 'S' {
                start = column as Column;
            }
            if char == '^' {
                let splitter = Splitter {
                    height: height as Height,
                    column: column as Column,
                };
                if let Some(splitters_column) = splitter_map.get_mut(&(column as Column)) {
                    splitters_column.push(splitter);
                } else {
                    let splitters_column: Vec<Splitter> = vec![splitter];
                    splitter_map.insert(column as Column, splitters_column);
                }
            }
        }
    }

    (start, splitter_map)
}

fn solve(input: ParsedInput) -> (usize, u64) {
    let (start, splitter_map) = input;

    let start_beam = Beam {
        start: 0,
        column: start,
        origin: None,
        side: None,
    };

    let mut tree: SplitterTree = SplitterTree {
        root: None,
        indices: HashMap::new(),
        nodes: Vec::new(),
    };

    let mut beams: VecDeque<Beam> = VecDeque::new();
    beams.push_back(start_beam);

    while !beams.is_empty() {
        let beam = beams.pop_front().unwrap();
        if let Some(column_splitters) = splitter_map.get(&beam.column) {
            for splitter in column_splitters.iter() {
                if splitter.height > beam.start {
                    if !tree.contains(splitter) {
                        let node = TreeNode {
                            splitter: *splitter,
                            left_child: None,
                            right_child: None,
                        };
                        tree.insert(node).unwrap();

                        let beam_left = Beam {
                            start: splitter.height,
                            column: splitter.column - 1,
                            origin: Some(*splitter),
                            side: Some(Side::Left),
                        };
                        beams.push_back(beam_left);

                        let beam_right = Beam {
                            start: splitter.height,
                            column: splitter.column + 1,
                            origin: Some(*splitter),
                            side: Some(Side::Right),
                        };
                        beams.push_back(beam_right);
                    }

                    if let Some(parent) = beam.origin
                        && let Some(side) = beam.side
                    {
                        tree.connect(parent, *splitter, side).unwrap();
                    }

                    break;
                }
            }
        }
    }

    (tree.size(), collapse_the_wavefunction(tree))
}

fn collapse_the_wavefunction(tree: SplitterTree) -> u64 {
    let mut cache: HashMap<Splitter, u64> = HashMap::new();
    tree_walk(&tree, tree.root.unwrap(), &mut cache)
}

fn tree_walk(tree: &SplitterTree, splitter: Splitter, cache: &mut HashMap<Splitter, u64>) -> u64 {
    if let Some(cached) = cache.get(&splitter) {
        return *cached;
    }

    let node = tree.get(&splitter).unwrap();

    if node.left_child.is_none() && node.right_child.is_none() {
        return 2;
    }

    let mut left: u64 = 1;
    let mut right: u64 = 1;

    if let Some(child) = node.left_child {
        left = tree_walk(tree, child, cache);
    }
    if let Some(child) = node.right_child {
        right = tree_walk(tree, child, cache);
    }

    let total = left + right;

    cache.insert(splitter, total);

    total
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
        assert_eq!(part_2, 40);
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
        assert_eq!(part_2, 95408386769474);
    }
}
