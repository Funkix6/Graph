/*
TODO:
  - Add Adjacency matrix
  - Add Incidence matrix
  - Refactor code with everything I learned along the way
*/

pub struct Graph {
    vertices: Vec<Vertex>,
    edges: Vec<Edge>,
}

#[derive(Clone)]
struct Vertex {
    index: i32,
    value: i32,
    neighbors: Vec<i32>,
}

#[derive(Copy, Clone)]
struct Edge {
    pair: (i32, i32),
    weight: i32,
}

impl Graph {
    pub const fn new() -> Self {
        Self {
            vertices: vec![],
            edges: vec![],
        }
    }

    pub fn add_vertex(&mut self, index: i32, value: i32) {
        if self.vertices.contains(&Vertex::new(index, 0)) {
            println!("Graph already contains vertex {}.", index);
        } else {
            self.vertices.push(Vertex::new(index, value));
        }
    }

    pub fn remove_vertex(&mut self, index: i32) {
        //Remove linked edges
        if let Some(vertex_index) = self
            .vertices
            .iter()
            .position(|x| x == &Vertex::new(index, 0))
        {
            for i in 1..self.edges.len() {
                self.remove_edge(index, i.try_into().unwrap());
                self.remove_edge(i.try_into().unwrap(), index);
            }
            self.vertices.remove(vertex_index);
            self.update_neighbors();
            println!("Deleted vertex : {}", index);
        } else {
            println!("Vertex not found : {}", index);
        }
    }

    pub fn get_vertices(&self) -> Vec<i32> {
        let mut vertices: Vec<i32> = vec![];
        for vertex in &self.vertices {
            vertices.push(vertex.index);
        }
        vertices
    }

    pub fn add_edge(&mut self, vertex_a: i32, vertex_b: i32, weight: i32) {
        if !self.get_vertices().contains(&vertex_a) || !self.get_vertices().contains(&vertex_b) {
            println!("At least one of the vertices is not in the graph");
        } else if self.edges.contains(&Edge::new(vertex_a, vertex_b, weight)) {
            println!(
                "Graph already contains edge with pair {:?}.",
                (vertex_a, vertex_b)
            );
        } else {
            self.edges.push(Edge::new(vertex_a, vertex_b, weight));
            self.update_neighbors();
        }
    }

    pub fn remove_edge(&mut self, from: i32, to: i32) {
        if let Some(index) = self.edges.iter().position(|x| x == &Edge::new(from, to, 0)) {
            self.edges.remove(index);
            self.update_neighbors();
            println!("Deleted edge : {:?}", (from, to));
        } else {
            println!("Edge not found : {:?}", (from, to));
        }
    }

    pub fn get_edges(&self) -> Vec<(i32, i32)> {
        let mut edges: Vec<(i32, i32)> = vec![];
        for edge in &self.edges {
            edges.push((edge.pair.0, edge.pair.1));
        }
        edges
    }

    pub fn adjacent(&self, from: i32, to: i32) -> bool {
        self.get_edges().contains(&(from, to))
    }

    pub fn get_neighbors(&self, from: i32) -> Vec<i32> {
        let mut neighbors = vec![];
        for edge in &self.edges {
            if edge.pair.0 == from {
                neighbors.push(edge.pair.1);
            }
        }
        neighbors
    }

    pub fn update_neighbors(&mut self) {
        for i in 0..self.vertices.len() {
            self.vertices[i].neighbors = self.get_neighbors(self.vertices[i].index);
        }
    }
  
    pub fn get_edge_value(&self, from: i32, to: i32) -> i32 {
        self.edges.iter().position(|x| x == &Edge::new(from, to, 0)).map_or_else(|| {
          println!("This edge does not exist.");
          0
        }, |index| self.edges[index].weight)
    }

  

    pub fn set_edge_value(&mut self, from: i32, to: i32, new_value: i32) {
        self.edges.iter().position(|x| x == &Edge::new(from, to, 0)).map_or_else(||
          println!("This edge does not exist."),
          |index| self.edges[index].weight = new_value);
    }

    pub fn get_vertex_value(&mut self, index: i32) -> i32 {
        self.vertices.iter().position(|x| x == &Vertex::new(index, 0)).map_or_else(|| {
          println!("This vertex does not exist.");
          0
        }, |index| self.vertices[index].value)
    }

    pub fn set_vertex_value(&mut self, index: i32, new_value: i32) {
        self.vertices.iter().position(|x| x == &Vertex::new(index, 0)).map_or_else(|| 
          println!("This vertex does not exist."),
          |index| self.vertices[index].value = new_value);
    }

    pub fn print_adjency_list(&self) {
        for i in 1..self.vertices.len() {
            println!("{:?}", self.get_neighbors(i.try_into().unwrap()));
        }
    }
}

impl Vertex {
    pub const fn new(index: i32, value: i32) -> Self {
        Self {
            index,
            value,
            neighbors: vec![],
        }
    }
}

impl PartialEq for Vertex {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}

impl Edge {
    pub const fn new(vertex_a: i32, vertex_b: i32, weight: i32) -> Self {
        Self {
            pair: (vertex_a, vertex_b),
            weight,
        }
    }
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        self.pair.0 == other.pair.0 && self.pair.1 == other.pair.1
    }
}
