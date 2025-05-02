//This mod uses methods to load in the data and process it, printing out the final similarity score result at the end
mod graph;
mod analysis;

use graph::WebGraph;
use analysis::compare_subgraphs;
//no input and no output, the main goal is to print the similarity score at the end 
fn main() {
    let path = "web-BerkStan (2).txt"; 
    let threshold = 300000; // Assumption that Berkeley < 300000, Stanford >= 300000

    let graph = WebGraph::from_file(path).expect("Failed to read graph file"); 
    println!("Total nodes: {}", graph.node_count());
    println!("Total edges: {}", graph.edge_count());

    let (berkeley, stanford) = graph.split_by_threshold(threshold); //splits the graph by the given threshold
    println!("Berkeley nodes: {}", berkeley.node_count());
    println!("Stanford nodes: {}", stanford.node_count());

    let similarity = compare_subgraphs(&berkeley, &stanford); //calculates the similarity score between the websites
    println!("Similarity score: {:.4}", similarity);
}

