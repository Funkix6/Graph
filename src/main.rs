mod graph;

use crate::graph::Graph;

fn main() {
    let mut my_graph = Graph::new();


    // Creating vertices
    for i in 1..10 {
      my_graph.add_vertex(i, i);
    }


    // Creating edges from existing vertices
    for i in 1..10 {
      my_graph.add_edge(i, i+1, 1);
      my_graph.add_edge(i, i+2, 1);
      my_graph.add_edge(i, i+3, 1);
    }  
  
    my_graph.update_neighbors();

    //Gets the neigbors of all vertices starting by the first.
    for i in 1..10 {
      println!("{:?}", my_graph.get_neighbors(i));
    }

    my_graph.remove_vertex(4);
    my_graph.remove_edge(1, 2);
    my_graph.print_adjency_list();
}