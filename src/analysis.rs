use crate::graph::WebGraph;
use std::collections::HashSet;

pub fn compare_subgraphs(a: &WebGraph, b: &WebGraph) -> f64 {
    let a_links = a.all_out_links();
    let b_links = b.all_out_links();

    let intersection: HashSet<_> = a_links.intersection(&b_links).collect();
    let union: HashSet<_> = a_links.union(&b_links).collect();

    if union.is_empty() {
        return 0.0;
    }

    intersection.len() as f64 / union.len() as f64
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::WebGraph;
    use std::collections::{HashMap, HashSet};

    fn mock_graph(edges: &[(u32, Vec<u32>)]) -> WebGraph {
        let mut map = HashMap::new();
        let mut nodes = HashSet::new();
        for (from, to_list) in edges {
            map.insert(*from, to_list.clone());
            nodes.insert(*from);
            nodes.extend(to_list);
        }
        WebGraph { edges: map, nodes }
    }

    #[test]
    fn test_similarity() {
        let g1 = mock_graph(&[(1, vec![2, 3]), (2, vec![4])]);
        let g2 = mock_graph(&[(10, vec![2, 4]), (11, vec![5])]);
        let score = compare_subgraphs(&g1, &g2);
        assert!((score - 0.5).abs() < 1e-6);
    }

    #[test]
    fn test_split_by_threshold_includes_cross_links() {
        let g = mock_graph(&[(1, vec![2, 1000]), (1000, vec![1])]);
        let (berkeley, stanford) = g.split_by_threshold(1000);

        assert!(berkeley.edges.contains_key(&1));
        assert_eq!(berkeley.edges[&1], vec![2, 1000]); 

        assert!(stanford.edges.contains_key(&1000));
        assert_eq!(stanford.edges[&1000], vec![1]); 

        assert!(berkeley.nodes.contains(&1000)); 
        assert!(stanford.nodes.contains(&1)); 
    }
} 
