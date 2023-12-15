// main.rs
mod network_centrality; 
mod graph_construction; 
use network_centrality::calculate_betweenness_centrality; 
use petgraph::prelude::*;
use petgraph::graph::{Graph, NodeIndex};
use std::fs::File; 
use std::io::Write;
use tempfile::tempdir;

fn main() {
    let file_path = "results.csv"; 
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

}


#[cfg(test)]
mod tests {
    use super::*;
    use petgraph::visit::NodeIndexable;

    #[test]
    fn test_graph_construction() {
        // Create a sample CSV file contents for testing
        let contents = "home_team,away_team,home_goals,away_goals,result,season\nTeamA,TeamB,2,1,H,2021\nTeamB,TeamC,0,3,A,2021\n";
        
        // Create a temporary directory for the test file
        let dir = tempdir().expect("Failed to create temporary directory");
    
        // Specify the file path within the temporary directory
        let file_path = dir.path().join("results.csv");
    
        // Convert the file_path to a &str
        let file_path_str = file_path.to_str().expect("Failed to convert file path to string");
    
        // Write the test data to the file
        let mut file = File::create(&file_path).expect("Failed to create file");
        writeln!(file, "{}", contents).expect("Failed to write to file");
    
        // Construct the graph
        let graph = graph_construction::construct_graph(file_path_str);
    
        // Verify that the graph has the expected number of nodes and edges based on the sample data
        assert_eq!(graph.node_count(), 3); // Number of unique teams
        assert_eq!(graph.edge_count(), 2); // Number of matches
    }
    
    

    #[test]
    fn test_centrality_calculation() {
        // Construct a test graph manually.
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