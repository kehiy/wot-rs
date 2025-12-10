use petgraph::graph::Graph;
use petgraph::visit::EdgeRef;

pub fn dump(graph: &Graph<(), ()>) {
    println!("Graph:");
    println!("  node_count = {}", graph.node_count());
    println!("  edge_count = {}", graph.edge_count());
    println!();

    println!("Nodes:");
    for node in graph.node_indices() {
        println!("  - {:?}", node);
    }
    println!();

    println!("Edges:");
    for edge in graph.edge_references() {
        let src = edge.source();
        let dst = edge.target();
        println!("  - {:?} -> {:?}", src, dst);
    }
}
