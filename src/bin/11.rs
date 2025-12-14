use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use memoize::memoize;
use rayon::prelude::*;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "11";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out
";

const TEST_2: &str = "\
svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out
";
struct DirectedGraph {
    matrix: Vec<Vec<bool>>,
    edge_names: HashMap<String, usize>,
    edge_index: Vec<String>,
    size: usize,
}

impl DirectedGraph {
    fn new(size: usize, edge_index: Vec<String>) -> Self {
        let mut hashmap = HashMap::new();
        edge_index.iter().enumerate().for_each(|(i, val)| {
            hashmap.insert(val.clone(), i);
        });

        DirectedGraph {
            matrix: vec![vec![false; size]; size],
            edge_names: hashmap,
            edge_index,
            size,
        }
    }

    fn add_edge_by_name(&mut self, a: &str, b: &str) -> Result<()> {
        match (self.get(a), self.get(b)) {
            (Some(a_index), Some(b_index)) => {
                self.matrix[a_index][b_index] = true;
                Ok(())
            }
            (None, Some(_)) => Err(anyhow!("'{}' not found in edges names", a)),
            (Some(_), None) => Err(anyhow!("'{}' not found in edges names", b)),
            (None, None) => Err(anyhow!("'{}' and '{}' not found in edges names", a, b)),
        }
    }

    fn get_neighbors(&self, node: usize) -> Vec<usize> {
        (0..self.size).filter(|&i| self.matrix[node][i]).collect()
    }

    fn get(&self, node: &str) -> Option<usize> {
        self.edge_names.get(node).cloned()
    }

    fn count_paths(&self, start: usize, end: usize, visited: &mut HashSet<usize>) -> usize {
        enum Action {
            Visit(usize),
            Backtrack(usize),
        }

        let mut stack = vec![Action::Visit(start)];
        let mut count = 0;

        while let Some(action) = stack.pop() {
            match action {
                Action::Visit(current) => {
                    if current == end {
                        count += 1;
                        continue;
                    }

                    if visited.contains(&current) {
                        continue;
                    }

                    visited.insert(current);
                    stack.push(Action::Backtrack(current));

                    for neighbor in self.get_neighbors(current) {
                        if !visited.contains(&neighbor) {
                            stack.push(Action::Visit(neighbor));
                        }
                    }
                }
                Action::Backtrack(node) => {
                    visited.remove(&node);
                }
            }
        }

        count
    }

    fn count_paths_optimized(&self, start: usize, end: usize) -> usize {
        let mut memo: HashMap<usize, usize> = HashMap::new();
        
        fn dfs_memo(
            graph: &DirectedGraph,
            current: usize,
            end: usize,
            memo: &mut HashMap<usize, usize>,
            visited: &mut HashSet<usize>,
        ) -> usize {
            if current == end {
                return 1;
            }
            
            if visited.contains(&current) {
                return 0;
            }
            
            if let Some(&cached) = memo.get(&current) {
                return cached;
            }
            
            visited.insert(current);
            let mut count = 0;
            
            for neighbor in graph.get_neighbors(current) {
                count += dfs_memo(graph, neighbor, end, memo, visited);
            }
            
            visited.remove(&current);
            memo.insert(current, count);
            count
        }
        
        let mut visited = HashSet::new();
        dfs_memo(self, start, end, &mut memo, &mut visited)
    }
}
fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let graph = create_graph(reader)?;

        let mut hashset = HashSet::new();
        let path_count = graph.count_paths(
            graph.get("you").unwrap(),
            graph.get("out").unwrap(),
            &mut hashset,
        );

        Ok(path_count)
    }
    fn create_graph<R: BufRead>(reader: R) -> Result<DirectedGraph> {
        let lines = reader.lines().flatten().collect_vec();

        let mut edge_names = Vec::new();
        for line in lines.iter() {
            let node_name = line.split(':').collect_vec()[0].to_string();
            edge_names.push(node_name);
        }
        edge_names.push("out".to_string());

        let mut graph = DirectedGraph::new(lines.len() + 1, edge_names);

        for line in lines {
            let splitted = line.split(':').collect_vec();
            let node_name = splitted[0].to_string();
            let connected_to = splitted[1].trim().split(' ').collect_vec();

            for connect in connected_to {
                graph.add_edge_by_name(&node_name, connect)?;
            }
        }
        Ok(graph)
    }

    assert_eq!(5, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let graph = create_graph(reader)?;

        // Path segmentation: svr -> fft -> dac -> out
        // Total paths = paths(svr->fft) × paths(fft->dac) × paths(dac->out)

        let svr = graph.get("svr").unwrap();
        let fft = graph.get("fft").unwrap();
        let dac = graph.get("dac").unwrap();
        let out = graph.get("out").unwrap();

        let path_specs = vec![
            (svr, fft, "Path 1"),
            (fft, dac, "Path 2"),
            (dac, out, "Path 3"),
        ];

        println!("\n--- Running ---");
        let results: Vec<usize> = path_specs
            .par_iter()
            .map(|&(start, end, label)| {
                println!("'{}' started", label);
                let count = graph.count_paths_optimized(start, end);
                println!("'{}' finished {} paths", label, count);
                count
            })
            .collect();

        let total_memo = results.iter().product();

        Ok(total_memo)
    }

    assert_eq!(2, part2(BufReader::new(TEST_2.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
