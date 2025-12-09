use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use regex::Regex;
use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "08";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
";

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Vector3 {
    x: usize,
    y: usize,
    z: usize,
}

impl Vector3 {
    fn new(x: usize, y: usize, z: usize) -> Self {
        Self { x, y, z }
    }

    fn euclidean_distance_with(self, other: Self) -> usize {
        let x = (self.x as i32 - other.x as i32).abs() as usize;
        let y = (self.y as i32 - other.y as i32).abs() as usize;
        let z = (self.z as i32 - other.z as i32).abs() as usize;
        (x.pow(2) + y.pow(2) + z.pow(2)).isqrt()
    }
}

impl Display for Vector3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{},{})", self.x, self.y, self.z)
    }
}

struct Graph {
    matrix: Vec<Vec<bool>>,
    size: usize,
}

impl Graph {
    fn new(size: usize) -> Self {
        Graph {
            matrix: vec![vec![false; size]; size],
            size,
        }
    }

    fn add_edge(&mut self, a: usize, b: usize) {
        self.matrix[a][b] = true;
        self.matrix[b][a] = true;
    }

    fn neighbors(&self, node: usize) -> Vec<usize> {
        (0..self.size).filter(|&i| self.matrix[node][i]).collect()
    }

    // DFS to find all nodes in a connected component
    fn find_component(&self, start: usize, visited: &mut HashSet<usize>) -> usize {
        let mut stack = vec![start];
        let mut count = 0;

        while let Some(node) = stack.pop() {
            if visited.insert(node) {
                count += 1;
                for neighbor in self.neighbors(node) {
                    if !visited.contains(&neighbor) {
                        stack.push(neighbor);
                    }
                }
            }
        }

        count
    }
    fn get_circuit_sizes(&self) -> Vec<usize> {
        let mut visited = HashSet::new();
        let mut sizes = Vec::new();

        for node in 0..self.size {
            if !visited.contains(&node) {
                let size = self.find_component(node, &mut visited);
                sizes.push(size);
            }
        }

        sizes
    }
}

#[derive(Clone, Copy, Debug)]
struct VectorDistance {
    a: usize,
    b: usize,
    distance: usize,
}

#[derive(Debug, Clone)]
struct UnionSet {
    parent: Vec<usize>,
    rank: Vec<u8>,
    components: usize,
}

impl UnionSet {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n],
            components: n,
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            let p = self.parent[x];
            self.parent[x] = self.find(p);
        }
        self.parent[x]
    }

    fn union(&mut self, a: usize, b: usize) -> bool {
        let ra = self.find(a);
        let rb = self.find(b);

        if ra == rb {
            return false;
        }

        if self.rank[ra] < self.rank[rb] {
            self.parent[ra] = rb;
        } else if self.rank[rb] < self.rank[ra] {
            self.parent[rb] = ra;
        } else {
            self.parent[rb] = ra;
            self.rank[ra] += 1;
        }

        self.components -= 1;
        true
    }

    fn is_all_connected(&self) -> bool {
        self.components == 1
    }
}

fn get_sorted_distances_and_coords<R: BufRead>(
    reader: R,
) -> Result<(Vec<Vector3>, Vec<VectorDistance>)> {
    const DEFAULT_VALUE: usize = usize::MAX;

    let lines = reader.lines().flatten().collect_vec();
    let line_regex = Regex::new(r"(\d+),(\d+),(\d+)")?;
    let mut coords = Vec::new();

    for line in lines {
        let capture = line_regex.captures(&line);
        if capture.is_none() {
            println!("Failed to capture one line, skipping it");
            continue;
        }

        let capture = capture.unwrap();

        coords.push(Vector3::new(
            capture[1].parse()?,
            capture[2].parse()?,
            capture[3].parse()?,
        ));
    }

    //println!("Coords: {:?}", coords);

    let mut distance_matrix = Vec::new();
    for i in 0..coords.len() {
        distance_matrix.push(
            (0..i)
                .map(|f| coords[i].euclidean_distance_with(coords[f]))
                .chain((i..coords.len()).map(|_| DEFAULT_VALUE))
                .collect_vec(),
        )
    }

    let mut sorted_distances: Vec<VectorDistance> = Vec::new();
    // Mirror the distance_matrix and create the sorted distances
    for i in 0..distance_matrix.len() {
        for j in (i + 1)..distance_matrix[i].len() {
            distance_matrix[i][j] = distance_matrix[j][i];
            sorted_distances.push(VectorDistance {
                a: i,
                b: j,
                distance: distance_matrix[i][j],
            });
        }
    }

    sorted_distances.sort_by_key(|d| d.distance);

    Ok((coords.clone(), sorted_distances.clone()))
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R, number_to_connect: usize) -> Result<usize> {
        let (coords, sorted_distances) = get_sorted_distances_and_coords(reader)?;
        let mut graph = Graph::new(coords.len());

        for i in 0..number_to_connect {
            let vec_distance = sorted_distances[i];
            //println!("{:?}", vec_distance);
            graph.add_edge(vec_distance.a, vec_distance.b);
        }

        let mut all_paths = graph.get_circuit_sizes();
        all_paths.sort_unstable_by(|a, b| b.cmp(a));
        let three_largest_paths = all_paths.iter().take(3).copied().collect_vec();

        Ok(three_largest_paths.iter().product())
    }

    assert_eq!(40, part1(BufReader::new(TEST.as_bytes()), 10)?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file, 1_000)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let (coords, sorted_distances) = get_sorted_distances_and_coords(reader)?;
        let mut graph = UnionSet::new(coords.len());

        let mut i = 0;
        while i < sorted_distances.len() {
            let vec_distance = sorted_distances[i];
            graph.union(vec_distance.a, vec_distance.b);
            if graph.is_all_connected() {
                let (vec1, vec2) = (coords[vec_distance.a], coords[vec_distance.b]);
                println!("{} with {}", vec1, vec2);
                return Ok(vec1.x * vec2.x);
            }
            i += 1;
        }

        Err(anyhow!("No solution found"))
    }

    assert_eq!(25272, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
