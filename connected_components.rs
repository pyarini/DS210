use crate::graph;
use crate::graph::Graph;
use std::collections::VecDeque;
pub type Vertex = usize;
pub type Component = usize;

#[derive(Debug, Clone)]
// create a structure for the connected components within the graph defining components and vertices of the largest component
pub struct ConnectedComponents {
    pub components: Vec<usize>,
    pub largest_component_vertices: Vec<Vertex>,
}
// BFS to mark the connected component of a given vertex
pub fn mark_component_bfs(vertex: Vertex, graph: &Graph, component: &mut Vec<Option<Component>>, component_no: Component) {
    component[vertex] = Some(component_no);
    let mut queue = VecDeque::new();
    queue.push_back(vertex);

    while let Some(v) = queue.pop_front() {
        for w_str in &graph.outedges[v] {
            if let Ok(w_index) = w_str.parse::<usize>() {
                if let None = component[w_index] {
                    component[w_index] = Some(component_no);
                    queue.push_back(w_index);
                }
            }
        }
    }
} 
impl ConnectedComponents {
    // create a new graph for Connected Components within the existing graph
    pub fn new(graph: graph::Graph) -> Self {
        let n = graph.n;
        let mut component: Vec<Option<Component>> = vec![None;n];
        let mut component_count = 0;
        
        // traverse vertices and mark connected components using BFS
        for vertex in 0..graph.outedges.len() {
            if component[vertex] == Some(0) {
                component_count += 1;
                mark_component_bfs(vertex, &graph, &mut component, component_count);
            }
        }
        let largest_component_vertices = Vec::new(); 
        Self {
            components: component.into_iter().map(|opt| opt.unwrap_or_default()).collect(),
            largest_component_vertices,
        }  
    }
    // get the vertices belonging to the largest connected component
    pub fn get_largest_component(&mut self) -> Vec<Vertex> {
        self.largest_component_vertices = Vec::new();
        let mut component_sizes = vec![0; self.components.len()];
    
        for &component in &self.components {
            component_sizes[component] += 1;
        }        
    
        if let Some(&max_size) = component_sizes.iter().max() {
            for (vertex, &component) in self.components.iter().enumerate() {
                if component_sizes[component] == max_size {
                    self.largest_component_vertices.push(vertex);
                }
            }
        }
        // return list of vertices in the largest connected component
    self.largest_component_vertices.clone()
}  
}      




