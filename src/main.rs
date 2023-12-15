mod network_centrality;
mod graph_construction;
use network_centrality::calculate_betweenness_centrality;
use petgraph::prelude::*;
use std::fs::File;
use std::io::Write;
use tempfile::tempdir;

fn main() {
    let file_path = "stats.csv";
    let graph = graph_construction::construct_graph(file_path);
    let teams = graph.node_indices().map(|node| graph.node_weight(node).unwrap().clone()).collect::<Vec<_>>();
    let count = graph.node_count();

    let mut centrality_scores: Vec<(String, f64)> = Vec::with_capacity(count);
    for v_index in 0..count {
        let centrality = calculate_betweenness_centrality(&graph, NodeIndex::new(v_index))[v_index];
        centrality_scores.push((teams[v_index].clone(), centrality));
    }

    centrality_scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    for (team, score) in centrality_scores {
        println!("Team: {}, Centrality: {}", team, score);
    }

    // Print the nodes
    println!("Graph Nodes:");
    for node_index in graph.node_indices() {
        if let Some(node_weight) = graph.node_weight(node_index) {
            println!("Node Index: {:?}, Team: {}", node_index.index(), node_weight);
        }
    }

    // Print the edges
    println!("Graph Edges:");
    for edge in graph.edge_indices() {
        let (source, target) = graph.edge_endpoints(edge).unwrap();
        let edge_weight = graph.edge_weight(edge).unwrap();
        println!("Edge from Node {} to Node {} with weight {}", source.index(), target.index(), edge_weight);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use petgraph::visit::NodeIndexable;

    #[test]
    fn test_graph_construction() {
        let contents = "team,wins,losses,goals,season\nTeamA,5,3,12,2021\nTeamB,7,2,18,2021\nTeamC,3,6,9,2021\n";

    }
    
    

    #[test]
    fn test_centrality_calculation() {
        // Construct a test graph 
        let mut graph = Graph::<String, i32>::new();
        let nodes = (1..=5)
            .map(|i| graph.add_node(i.to_string()))
            .collect::<Vec<_>>();
        graph.extend_with_edges(&[
            (nodes[0], nodes[1]),
            (nodes[1], nodes[2]),
            (nodes[2], nodes[3]),
            (nodes[3], nodes[4]),
            (nodes[0], nodes[2]),
            (nodes[0], nodes[3]),
            (nodes[1], nodes[3]),
            (nodes[2], nodes[4]),
        ]);

        // Expected centrality scores for each node.
        let expected_scores = vec![2.5, 2.0, 2.0, 1.0, 0.0];

        // Calculate centrality and verify against expected scores.
        for (i, &node) in nodes.iter().enumerate() {
            let centrality_scores = calculate_betweenness_centrality(&graph, node);
            assert_eq!(centrality_scores[i], expected_scores[i]);
        }
    }
}