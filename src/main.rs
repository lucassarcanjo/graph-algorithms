mod graph;
use graph::Graph;

fn main() {
    println!("Hi, let's go to run BFS on a graph 🔥");

    let mut d = Graph::new(4);

    d.add_edge(0, 1);
    d.add_edge(0, 2);
    d.add_edge(1, 2);
    d.add_edge(2, 0);
    d.add_edge(2, 3);
    d.add_edge(3, 3);

    const START_VERTEX: usize = 2;

    println!("Starting from vertex {:?}", START_VERTEX);

    d.bfs(START_VERTEX);

    println!("\n\nDone ✅\n")
}
