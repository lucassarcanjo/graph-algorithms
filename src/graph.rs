use std::collections::LinkedList;

pub struct Graph {
  // Adjacency lists
  pub(crate) adj_list: Vec<LinkedList<usize>>,

  // Count of vertices
  pub(crate) vertex_count: usize,
}

impl Graph {
  pub fn new(vertex_qty: usize) -> Self {
    let list = vec![LinkedList::new(); vertex_qty];

    Self { vertex_count: vertex_qty, adj_list: list }
  }

  pub fn add_edge(&mut self, vertex: usize, value: usize) {
    self.adj_list[vertex].push_back(value);
  }

  pub fn bfs(&mut self, start_vertex: usize) {
    // TODO: check if the start vertice is valid
    
    // create a new vector of each vertice and initialized with non-visited
    let mut visited = vec![false; self.vertex_count];

    // create a queue for BFS
    let mut queue: LinkedList<usize> = LinkedList::new();

    // intialize for first vertice
    visited[start_vertex] = true;
    queue.push_front(start_vertex);
    
    while queue.len() > 0 {
      let maybe_current_vertex = queue.pop_front();
      
      let current_vertex = maybe_current_vertex.unwrap();

      print!("{:?} ", current_vertex);
      
      let list = &self.adj_list[current_vertex];

      for &value in list {

        if !visited[value] {
          visited[value] = true;
          queue.push_back(value)
        }
      }
    }
  }
}