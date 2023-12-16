use crate::centrality::calculate_centrality;
use connected_components::ConnectedComponents;
pub type Vertex = usize;
pub mod data;
pub mod graph;
pub mod connected_components;
use graph::Graph;
pub mod centrality;

#[test]
fn test() {
    match data::read_csv("tollywood-cinema.csv") {
        // reads CSV file and creates graph with appropriate vertices and nodes
        Ok((titles, composers, genres)) => {
            let igraph = graph::create_graph(titles.clone(), composers, genres);
            // find connected components algorithm on graph and create new graph
            let mut connected_components = ConnectedComponents::new(igraph.clone());

            // find edges of the new connected components graph 
            let largest_component_edges = connected_components.get_largest_component();
            let mut flattened_edges: Vec<(usize, String)> = vec![];
            
            // iterate thru vertices to find the largest edges to begin constructing a graph consisting of the largest component
            for idx in largest_component_edges.iter() {
                if *idx < titles.len() {
                    flattened_edges.push((*idx, titles[*idx].clone()));
                }
            }

            // create a graph with the largest component
            let largest_component_graph = Graph::create_directed(igraph.n, &flattened_edges);
            // compute centrality between vertices of the largest component, store the scores in a hashmap
            let centrality_scores = calculate_centrality(&largest_component_graph);
            
            // iterate through centrality scores hashmap to confirm that scores are either 0.0 or 0.00020777062123415748
            for (vertex, centrality) in centrality_scores.iter() {
                assert!(
                    *centrality == 0.0 || *centrality == 0.00020777062123415748,
                    "outlier centrality value for vertex {}: {}",
                    vertex,
                    centrality
                );
            }
        }
        Err(_) => todo!(),
    }
}

fn main() {
    match data::read_csv("tollywood-cinema.csv") {
        // reads CSV file and creates graph with appropriate vertices and nodes
        Ok((titles, composers, genres)) => {
            let graph = graph::create_graph(titles.clone(), composers, genres);
            // create a graph with the vertices being titles, and nodes being composers and genres
            let mut component: Vec<Option<graph::Component>> = vec![Some(0); graph.n];
            // create vector to store relevant component information 
            let mut component_count = 0;

            for v in 0..graph.n {
                // iterate through vertices and run breadth-first search to identify connected components within the graph
                if let None = component[v] {
                    component_count += 1;
                    connected_components::mark_component_bfs(v, &graph, &mut component, component_count);
                }
            }
             // Create ConnectedComponents graph, define new edges and flatten edges to identify largest component
            let mut connected_components = ConnectedComponents::new(graph.clone());
            let largest_component_edges = connected_components.get_largest_component();
            let mut flattened_edges: Vec<(usize, String)> = vec![];
    
            // filter edges that belong to the largest component only
            for idx in largest_component_edges.iter() {
                if *idx < titles.len() {
                    flattened_edges.push((*idx, titles[*idx].clone()));
                }
            }

            // create a graph with the largest connected component identified 
            let largest_component_graph = Graph::create_directed(graph.n, &flattened_edges);
            // compute centrality within the largest component vertices, store the scores in a hashmap
            let centrality_scores = calculate_centrality(&largest_component_graph);

            println!("Centrality Scores within Largest Component: {:?}", centrality_scores);
        }
        Err(err) => eprintln!("Error: {:?}", err),
    }
}
