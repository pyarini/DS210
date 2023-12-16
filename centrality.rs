use std::collections::HashMap;
use crate::graph::Graph;
pub type Vertex = usize;

pub fn calculate_centrality(graph: &Graph) -> HashMap<Vertex, f64> {
    // function stores centrality scores in a new hashmap 
    let mut centrality_scores = HashMap::new();
    let total_nodes = graph.outedges.len() as f64;

    for vertex in 0..graph.outedges.len() {
        // iterates through the edges of the graph, and finds the sum of inedges and outedges
        // the sum is then divided by the number of nodes - 1 to not include the current node
        let in_degree = graph.inedges[vertex].len();
        let out_degree = graph.outedges[vertex].len();
        let sum = in_degree as f64 + out_degree as f64;

        let centrality = sum / (total_nodes - 1.0);

        centrality_scores.insert(vertex, centrality);
    }

    centrality_scores
}
