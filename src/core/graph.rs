use std::collections::HashMap;

use crate::core::matrix::Matrix;

/// The Graph
#[derive(Debug, Default)]
pub struct Graph {
    pub vertices: HashMap<String, usize>,
    pub adj_matrix: Matrix,
}

impl Graph {

    pub fn new() -> Graph {
        Self {
            vertices: HashMap::new(),
            adj_matrix: Matrix::new(),
        }
    }

    pub fn add_edge(&mut self, name: String, from: String, to: String) {
        let from_index = self.add_vertex_if_not_exists(from);
        let to_index = self.add_vertex_if_not_exists(to);
        self.adj_matrix.set_option(from_index, to_index, Some(name))
    }


    /// Adds a vertex in the graph.
    /// If the vertex already exists, function does nothing.
    /// 
    /// Returns the index position of the vertex
    fn add_vertex_if_not_exists(&mut self, vertex: String) -> usize {
        let index = self.vertices.get(&vertex);
        match index {
            Some(index) => { *index },
            None => { 
                let index = self.vertices.len();
                self.vertices.insert(vertex, index);
                self.adj_matrix.add_row();
                index
            }
        }
    }

}