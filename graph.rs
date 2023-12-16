type ListOfEdges = Vec<(usize, String)>;
type AdjacencyLists = Vec<Vec<String>>;
pub type Component = usize;

#[derive(Debug, Clone)]
// structure representing a graph with adjacency lists for outgoing and incoming edges
pub struct Graph {
    pub n: usize,        
    pub outedges: AdjacencyLists,
    pub inedges: AdjacencyLists,
}
// reverses the direction of edges in a list.
fn reverse_edges(list: &ListOfEdges) -> ListOfEdges {
    let mut new_list = vec![];
    for (u, v) in list {
        let new_v = v.clone();
        new_list.push((*u,new_v));
    }
    new_list
}
impl Graph {
    pub fn new(n: usize, edges: &ListOfEdges) -> Self {
        // create a new graph with a specified number of vertices and edges
        let mut graph = Graph { n, outedges: vec![vec![]; n], inedges: vec![vec![]; n], };
        graph.add_directed_edges(edges);
        graph
    }
    // add directed edges to the graph
    pub fn add_directed_edges(&mut self, edges: &ListOfEdges) {
        for (v, title) in edges {
            self.outedges[*v].push(title.clone());
        }
    } 
    // sort outgoing edge lists for each vertex
    pub fn sort_graph_lists(&mut self) {
        for l in self.outedges.iter_mut() {
            l.sort();
        }
    }
    // create a directed graph with sorted adjacency lists from a list of edges   
    pub fn create_directed(n:usize,edges:&ListOfEdges) -> Graph {
        let mut g = Graph{n,outedges:vec![vec![];n],inedges:vec![vec![];n]};
        g.add_directed_edges(edges);
        g.sort_graph_lists();
        g                                        
    }
    // create an undirected graph with sorted adjacency lists from a list of edges
    pub fn create_undirected(n:usize,edges:&ListOfEdges) -> Graph {
        let mut g = Self::create_directed(n,edges);
        g.add_directed_edges(&reverse_edges(edges));
        g.sort_graph_lists();
        g                                        
    }
}
// create a graph from vectors of titles, composers, and genres
pub fn create_graph(titles: Vec<String>, mut composers: Vec<String>, mut genres: Vec<String>) -> Graph {
    let mut edges: ListOfEdges = Vec::new();

    // iterate through titles to create edges between composers/genres and titles
    for i in 0..titles.len() {
        let title = titles[i].clone();
        let composer_index = composers.iter().position(|x| x == &composers[i]).unwrap_or_else(|| {
            composers.push(composers[i].clone());
            composers.len() - 1
        });

        let genre_index = genres.iter().position(|x| x == &genres[i]).unwrap_or_else(|| {
            genres.push(genres[i].clone());
            genres.len() - 1
        });

        edges.push((composer_index, title.clone()));
        edges.push((genre_index, title));
    }

    Graph::new(composers.len() + genres.len(), &edges)
}
 