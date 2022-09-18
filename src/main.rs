mod graph;
use graph::Graph;

fn main() {
    println!("Hi, let's go to run BFS on a graph 🔥");

    let mut d = Graph::new(false);

    d.add_edge("A", "B");
    d.add_edge("C", "D");
    d.add_edge("B", "C");

    d.add_node("E");

    d.bfs("B");

    println!("\n\nDone ✅\n")
}
