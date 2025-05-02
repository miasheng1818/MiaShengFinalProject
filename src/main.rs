mod graph;
mod analysis;

use graph::WebGraph;
use analysis::compare_subgraphs;

fn main() {
    let path = "web-BerkStan (2).txt";
    let threshold = 300000; // Assumes Berkeley < 300000, Stanford >= 300000

    let graph = WebGraph::from_file(path).expect("Failed to read graph file");
    println!("Total nodes: {}", graph.node_count());
    println!("Total edges: {}", graph.edge_count());

    let (berkeley, stanford) = graph.split_by_threshold(threshold);
    println!("Berkeley nodes: {}", berkeley.node_count());
    println!("Stanford nodes: {}", stanford.node_count());

    let similarity = compare_subgraphs(&berkeley, &stanford);
    println!("Similarity score: {:.4}", similarity);
}

