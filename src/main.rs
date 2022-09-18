mod graph;
use graph::Graph;

fn main() {
    println!("Hi, let's go to run BFS on a graph 🔥");

    let mut d = Graph::new();
    
    d.add_edge("0".to_owned(), "1".to_owned());
    d.add_edge("0".to_owned(), "2".to_owned());
    d.add_edge("1".to_owned(), "2".to_owned());
    d.add_edge("2".to_owned(), "3".to_owned());
    d.add_edge("2".to_owned(), "0".to_owned());
    d.add_edge("3".to_owned(), "3".to_owned());

    const START_VERTEX: &str = "1";

    println!("Starting from vertex {:?}\n", START_VERTEX);

    d.bfs(START_VERTEX.to_owned());

    println!("\n\nDone ✅\n")
}
