use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufReader, BufRead, Result};

#[derive(Debug)]
pub struct WebGraph {
    pub edges: HashMap<u32, Vec<u32>>,
    pub nodes: HashSet<u32>,
}

impl WebGraph {
    pub fn from_file(path: &str) -> Result<Self> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        let mut edges: HashMap<u32, Vec<u32>> = HashMap::new();
        let mut nodes: HashSet<u32> = HashSet::new();

        for line in reader.lines() {
            let line = line?;
            if line.starts_with('#') {
                continue;
            }
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() != 2 {
                continue;
            }
            let from: u32 = parts[0].parse().unwrap_or(0);
            let to: u32 = parts[1].parse().unwrap_or(0);

            edges.entry(from).or_default().push(to);
            nodes.insert(from);
            nodes.insert(to);
        }

        Ok(WebGraph { edges, nodes })
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    pub fn edge_count(&self) -> usize {
        self.edges.values().map(|v| v.len()).sum()
    }

    pub fn out_degrees(&self) -> HashMap<u32, usize> {
        self.edges.iter().map(|(&node, targets)| (node, targets.len())).collect()
    }

    pub fn all_out_links(&self) -> HashSet<u32> {
        self.edges.values().flat_map(|v| v.iter().copied()).collect()
    }

    pub fn split_by_threshold(&self, threshold: u32) -> (WebGraph, WebGraph) {
        let mut berkeley_edges: HashMap<u32, Vec<u32>> = HashMap::new();
        let mut berkeley_nodes: HashSet<u32> = HashSet::new();

        let mut stanford_edges: HashMap<u32, Vec<u32>> = HashMap::new();
        let mut stanford_nodes: HashSet<u32> = HashSet::new();

        for (&from, tos) in &self.edges {
            if from < threshold {
                berkeley_edges.insert(from, tos.clone());
                berkeley_nodes.insert(from);
                berkeley_nodes.extend(tos);
            } else {
                stanford_edges.insert(from, tos.clone());
                stanford_nodes.insert(from);
                stanford_nodes.extend(tos);
            }
        }

        (
            WebGraph {
                edges: berkeley_edges,
                nodes: berkeley_nodes,
            },
            WebGraph {
                edges: stanford_edges,
                nodes: stanford_nodes,
            },
        )
    }
}
