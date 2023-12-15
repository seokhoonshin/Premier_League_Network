// read_and_construct.rs

use csv::ReaderBuilder;
use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::visit::NodeIndexable;
use serde::Deserialize;
use std::{collections::HashMap, fs::File, io::BufReader};

#[derive(Deserialize)]
struct Match {
    home_team: String,
    away_team: String,
    home_goals: f64,
    away_goals: f64,
    result: String, // 'H', 'A', 'D'
    season: String,
}

impl Match {
    // Function to determine the weight of an edge based on match result and goals
    fn match_weight(&self) -> i32 {
        let goal_diff = (self.home_goals - self.away_goals) as i32;
        match self.result.as_str() {
            "H" => goal_diff.max(1), // Home win, at least weight of 1
            "A" => (-goal_diff).max(1), // Away win, at least weight of 1
            "D" => 0, // Draw
            _ => 0, // Unexpected result, treat as draw
        }
    }
}

pub fn construct_graph(file_path: &str) -> DiGraph<String, i32> {
    let mut graph = DiGraph::<String, i32>::new();
    let mut name_map: HashMap<String, NodeIndex> = HashMap::new();

    let file = File::open(file_path).expect("Unable to open file");
    let reader = BufReader::new(file);
    let mut csv_reader = ReaderBuilder::new().has_headers(true).from_reader(reader);

    for record in csv_reader.deserialize() {
        let match_data: Match = match record {
            Ok(data) => data,
            Err(e) => {
                eprintln!("Error reading record: {}", e);
                continue;
            }
        };

        let home_node = *name_map.entry(match_data.home_team.clone())
                        .or_insert_with(|| graph.add_node(match_data.home_team.clone()));
        let away_node = *name_map.entry(match_data.away_team.clone())
                        .or_insert_with(|| graph.add_node(match_data.away_team.clone()));

        graph.add_edge(home_node, away_node, match_data.match_weight());
    }

    graph
}
