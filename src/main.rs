mod graph;
use graph::Graph;

fn main() {
    println!("Hi, let's go to run BFS and DFS on a graph 🔥");

    let mut d = Graph::new(false);

    d.add_edge("1", "0");
    d.add_edge("0", "2");
    d.add_edge("2", "1");
    d.add_edge("0", "3");
    d.add_edge("1", "4");

    d.add_node("5");

    d.bfs("0");
    d.dfs("0");

    println!("\n\nDone ✅\n")
}
