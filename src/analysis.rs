//this has the function to compare the similarities of 2 graphs and give a similarity score and it also conducts all of my test
use crate::graph::WebGraph;
use std::collections::HashSet;

pub fn compare_subgraphs(a: &WebGraph, b: &WebGraph) -> f64 { //computes a similarity score, takes in references to graphs of the different websites and outputs a float with the similarity score
    let a_links = a.all_out_links();
    let b_links = b.all_out_links();

    let intersection: HashSet<_> = a_links.intersection(&b_links).collect();
    let union: HashSet<_> = a_links.union(&b_links).collect();

    if union.is_empty() { //makes sure that we dont divide by 0
        return 0.0;
    }

    intersection.len() as f64 / union.len() as f64
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::WebGraph;
    use std::collections::{HashMap, HashSet};

    fn mock_graph(edges: &[(u32, Vec<u32>)]) -> WebGraph { //this creates a mock up graph to use for testing purposes
        let mut map = HashMap::new();
        let mut nodes = HashSet::new();
        for (from, to_list) in edges {
            map.insert(*from, to_list.clone());
            nodes.insert(*from);
            nodes.extend(to_list);
        }
        WebGraph { edges: map, nodes }
    }

    #[test] //this tests the similarity score algorithm by using fake data 
    fn test_similarity() {
        let g1 = mock_graph(&[(1, vec![2, 3]), (2, vec![4])]);
        let g2 = mock_graph(&[(10, vec![2, 4]), (11, vec![5])]);
        let score = compare_subgraphs(&g1, &g2);
        assert!((score - 0.5).abs() < 1e-6);
    }

    #[test]
    fn test_out_degrees() {
        let g = mock_graph(&[
            (1, vec![2, 3]),
            (2, vec![3]),
            (3, vec![]),
        ]);

        let out_degrees = g.out_degrees();

        assert_eq!(out_degrees.get(&1), Some(&2)); // Node 1 has 2 outgoing edges
        assert_eq!(out_degrees.get(&2), Some(&1)); // Node 2 has 1 outgoing edge
        assert_eq!(out_degrees.get(&3), Some(&0)); // Node 3 has 0 outgoing edges
    }
}   
