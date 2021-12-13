use std::cmp::{max, min, Ordering};
use std::collections::HashMap;
use std::str::FromStr;

use anyhow::Result;

pub const INPUT_PATH: &str = "inputs/day/12/input";

pub fn part_one(contents: &str) -> Result<usize> {
    let edges = load_edges(contents)?;
    Ok(find_paths(edges.as_slice()).len())
}

pub fn part_two(contents: &str) -> Result<usize> {
    let edges = load_edges(contents)?;
    Ok(find_paths_with_small(edges.as_slice()).len())
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum Vertex {
    Start,
    End,
    Cave(String, bool),
}

impl Vertex {
    fn is_small(&self) -> bool {
        use Vertex::*;
        match self {
            Start | End => true,
            Cave(_, small) => *small,
        }
    }
}

impl FromStr for Vertex {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "start" {
            Ok(Vertex::Start)
        } else if s == "end" {
            Ok(Vertex::End)
        } else {
            Ok(Vertex::Cave(
                s.to_string(),
                s.as_bytes().iter().all(|c| c.is_ascii_lowercase()),
            ))
        }
    }
}

impl Ord for Vertex {
    fn cmp(&self, other: &Self) -> Ordering {
        use Ordering::*;
        match self {
            Vertex::Start => {
                if *other == Vertex::Start {
                    Equal
                } else {
                    Less
                }
            }
            Vertex::End => {
                if *other == Vertex::End {
                    Equal
                } else {
                    Greater
                }
            }
            Vertex::Cave(left, _) => match other {
                Vertex::Start => Greater,
                Vertex::End => Less,
                Vertex::Cave(right, _) => left.to_lowercase().cmp(&right.to_lowercase()),
            },
        }
    }
}

impl PartialOrd for Vertex {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn find_paths(edges: &[(Vertex, Vertex)]) -> Vec<Vec<Vertex>> {
    let mut graph: HashMap<Vertex, Vec<Vertex>> = HashMap::new();
    let mut paths = Vec::new();
    edges
        .iter()
        .flat_map(|(v1, v2)| vec![(v1, v2), (v2, v1)])
        .for_each(|(left, right)| {
            let vertices = graph.entry(left.clone()).or_insert_with(Vec::new);
            vertices.push(right.clone())
        });

    let mut queue = vec![vec![Vertex::Start]];

    while !queue.is_empty() {
        let path = queue
            .pop()
            .expect("queue always has a value per while condition");
        let v = path.iter().last().expect("empty path");
        if *v == Vertex::End {
            paths.push(path);
            continue;
        }

        graph
            .get(v)
            .expect("graph is bidirectional so there should always be at least a reverse edge")
            .iter()
            .for_each(|v| {
                if !v.is_small() || !path.contains(v) {
                    let mut next_path = path.clone();
                    next_path.push(v.clone());
                    queue.push(next_path);
                }
            });
    }

    paths
}

fn find_paths_with_small(edges: &[(Vertex, Vertex)]) -> Vec<Vec<Vertex>> {
    let mut graph: HashMap<Vertex, Vec<Vertex>> = HashMap::new();
    let mut paths = Vec::new();
    edges
        .iter()
        .flat_map(|(v1, v2)| vec![(v1, v2), (v2, v1)])
        .for_each(|(left, right)| {
            let vertices = graph.entry(left.clone()).or_insert_with(Vec::new);
            vertices.push(right.clone())
        });

    // TODO: this can be simpler, just (Path, bool)
    // Then allow the caller to initialize the bool and replace find_paths
    let mut queue = vec![(vec![Vertex::Start], None)];

    while !queue.is_empty() {
        let (path, used_small): (Vec<Vertex>, Option<Vertex>) = queue
            .pop()
            .expect("queue always has a value per while condition");
        let v = path.iter().last().expect("empty path");
        if *v == Vertex::End {
            paths.push(path);
            continue;
        }

        graph
            .get(v)
            .expect("graph is bidirectional so there should always be at least a reverse edge")
            .iter()
            .filter(|v| Vertex::Start != **v)
            .for_each(|v| {
                let count = if v.is_small() {
                    path.iter().filter(|p_v| *p_v == v).count()
                } else {
                    0 // duplicates don't matter for large caves
                };

                if count == 0 {
                    let mut next_path = path.clone();
                    next_path.push(v.clone());
                    queue.push((next_path, used_small.clone()));
                } else if count == 1 && used_small.is_none() {
                    let mut next_path = path.clone();
                    next_path.push(v.clone());
                    let used_small = match &used_small {
                        Some(u) => Some(u.clone()),
                        None => Some(v.clone()), // v is always small because count > 0
                    };
                    queue.push((next_path, used_small));
                }
            });
    }

    paths
}

fn load_edges(contents: &str) -> Result<Vec<(Vertex, Vertex)>> {
    contents
        .lines()
        .flat_map(|edge| {
            edge.split_once('-')
                .map(|(left, right)| {
                    let v1 = Vertex::from_str(left)?;
                    let v2 = Vertex::from_str(right)?;
                    Ok((min(v1.clone(), v2.clone()), max(v1, v2)))
                })
                .ok_or_else(|| anyhow::anyhow!("bad input"))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn test_load_edges() {
        use Vertex::*;
        let expected = [
            (Start, Cave("A".into(), false)),
            (Start, Cave("b".into(), true)),
            (Cave("A".into(), false), Cave("c".into(), true)),
            (Cave("A".into(), false), Cave("b".into(), true)),
            (Cave("b".into(), true), Cave("d".into(), true)),
            (Cave("A".into(), false), End),
            (Cave("b".into(), true), End),
        ];
        assert_eq!(
            expected,
            load_edges(read_to_string("fixtures/simple_cave.txt").unwrap().as_str())
                .unwrap()
                .as_slice()
        )
    }

    #[test]
    fn test_find_paths() {
        let input = load_edges(read_to_string("fixtures/cave.txt").unwrap().as_str()).unwrap();
        assert_eq!(19, find_paths(input.as_slice()).len())
    }

    #[test]
    fn test_find_paths_with_small() {
        let input =
            load_edges(read_to_string("fixtures/simple_cave.txt").unwrap().as_str()).unwrap();
        assert_eq!(36, find_paths_with_small(input.as_slice()).len())
    }
}
