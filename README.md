# Analyzing the Premier League Network: DS210 Final Project Report

## Overview
This project applies network analysis techniques to explore the structure of the Premier League from the 2006/2007 season to the 2017/2018 season. By using Rust and advanced graph algorithms, the project highlights the hidden dynamics within the league and provides a novel perspective on team influence and interconnectedness.

---

## Key Features
- **Dataset:**
  - Premier League match results and statistics (2006/2007 to 2017/2018).
  - Contains data from 4560 matches, with 9120 edges and 9121 nodes.
  - Source: [Premier League Results and Stats](https://www.kaggle.com/datasets/zaeemnalla/premier-league).
- **Focus:**
  - Computation of Betweenness Centrality using Dijkstra's algorithm.
  - Analysis of team influence through network metrics.

---

## Methodology

### Graph Construction
- **Nodes:** Premier League teams.
- **Edges:** Represent match relationships (wins, losses, goals).
- **Edge Weights:** Calculated using the absolute difference in team wins to reflect competitive distance.

### Network Centrality Analysis
- Computed Betweenness Centrality using:
  - **Brandes' Algorithm**: For efficient centrality calculations.
  - **Dijkstra's Algorithm**: For shortest path computations.
- Tools: Rust programming language, `petgraph` library.

### Key Insights
- **Swansea City** emerged with the highest betweenness centrality score (5304.176).
  - Indicates its pivotal role in the network's dynamics.
- **Stoke City** and **Watford** also showed significant influence, challenging traditional notions of team dominance.
- Highlighted the strategic importance of teams in shaping the season's outcomes beyond mere match victories.

---

## Results
- Betweenness Centrality revealed the covert forces influencing the league.
- Swansea City's high centrality score suggests its matches play a crucial role in determining the league's structure.
- Teams with low centrality scores (e.g., Sheffield United) had limited influence on overall network dynamics.

---

## Tools and Technologies
- **Programming Language:** Rust
- **Libraries:** `petgraph`, `graphrs`
- **Algorithms:** Dijkstra’s Algorithm, Brandes’ Algorithm
- **Dataset Source:** Kaggle

---

