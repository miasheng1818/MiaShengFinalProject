//functions all have to do with the creation/processing of a website's graph with nodes and edges 
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufReader, BufRead, Result};

//represents a directed graph of the nodes and edges, one hashmap represents the edges and one represents the unique set of nodes in each graph 
#[derive(Debug)]
pub struct WebGraph {
    pub edges: HashMap<u32, Vec<u32>>,
    pub nodes: HashSet<u32>,
}

impl WebGraph {
    // processes the given file, it takes in a reference to a string which is the path and returns a webgraph 
    pub fn from_file(path: &str) -> Result<Self> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        let mut edges: HashMap<u32, Vec<u32>> = HashMap::new();
        let mut nodes: HashSet<u32> = HashSet::new();

        for line in reader.lines() {
            let line = line?;
            if line.starts_with('#') { //skips any comments
                continue;
            }
            let parts: Vec<&str> = line.split_whitespace().collect(); //only processes the data if it has a from value and a to value
            if parts.len() != 2 {
                continue;
            }
            let from: u32 = parts[0].parse().unwrap_or(0); //this parses the node identifiers and if the parsing fails then it defaults to a 0
            let to: u32 = parts[1].parse().unwrap_or(0);

            edges.entry(from).or_default().push(to); //adds in the edges and nodes
            nodes.insert(from);
            nodes.insert(to);
        }

        Ok(WebGraph { edges, nodes })
    }

    pub fn node_count(&self) -> usize { //counts unique nodes 
        self.nodes.len()
    }

    pub fn edge_count(&self) -> usize { //counts total edges 
        self.edges.values().map(|v| v.len()).sum()
    }

    pub fn out_degrees(&self) -> HashMap<u32, usize> { //hashmap of nodes and their outgoing edges 
        self.edges.iter().map(|(&node, targets)| (node, targets.len())).collect()
    }

    pub fn all_out_links(&self) -> HashSet<u32> { //returns a hashset of any node that has another node pointing to it 
        self.edges.values().flat_map(|v| v.iter().copied()).collect()
    }

    //this makes two subgraphs of each website, splitting it into berkeley and stanford 
    //takes in a threshold number and returns a tuple of 2 webgraphs, one for berk and one for stanford
    pub fn split_by_threshold(&self, threshold: u32) -> (WebGraph, WebGraph) {
        let mut berkeley_edges: HashMap<u32, Vec<u32>> = HashMap::new();
        let mut berkeley_nodes: HashSet<u32> = HashSet::new();

        let mut stanford_edges: HashMap<u32, Vec<u32>> = HashMap::new();
        let mut stanford_nodes: HashSet<u32> = HashSet::new();

        for (&from, tos) in &self.edges {
            if from < threshold { //adds to berkley graph
                berkeley_edges.insert(from, tos.clone());
                berkeley_nodes.insert(from);
                berkeley_nodes.extend(tos);
            } else { //adds to stanford graph
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
