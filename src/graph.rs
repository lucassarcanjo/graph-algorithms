use std::{collections::{LinkedList, VecDeque}, borrow::Borrow};

#[derive(Clone)]
pub struct Node {
  name: &'static str,
  // parent_name: &'static str,
  // level: i32,
  index: i32,
  visited: bool,
}

pub struct Adjacency {
  node: &'static str,
  list: LinkedList<&'static str>
}

pub struct Graph {
  pub(crate) adjacency_list: Vec<Adjacency>
}

impl Graph {
  pub fn new() -> Self {
    let adjacency_list = Vec::new();

    Self { adjacency_list }
  }

  pub fn add_edge(&mut self, from: &'static str, to: &'static str) {
    let from_index_result = self.get_node_position_by_name(from);

    let node_from_index = match from_index_result {
      // if node not exists yet at self.adjacency_list
      None => {
        let adj = Adjacency {
          node: from,
          list: LinkedList::new()
        };

        self.adjacency_list.push(adj);

        // return index of new item
        self.adjacency_list.len() - 1
      },
      // if node already exists
      Some(i) => i
    };

    let to_index_result = self.get_node_position_by_name(to);

    if to_index_result == None {
      let adj = Adjacency {
        node: to,
        list: LinkedList::new()
      };

      self.adjacency_list.push(adj);
    }

    self.adjacency_list[node_from_index].list.push_back(to);
  }

  pub fn bfs(&mut self, start_node_name: &str) {
    // check if the start node is valid
    let start_node_result = self.get_node_position_by_name(start_node_name);

    if start_node_result == None {
      println!("Invalid node, you must to add an edge with name: {} or add a node manually", start_node_name);
      return;
    }

    // initializations
    // let node_list_count = self.adjacency_list.len();
    let mut counter = 0;
    let mut queue: VecDeque<Node> = VecDeque::new();
    let mut node_list: Vec<Node> = Vec::with_capacity(self.adjacency_list.len());

    // node initialization
    for list_item in &self.adjacency_list {
      node_list.push(
        Node { 
          index: 0,
          // level: 0,
          // parent_name: "",
          name: list_item.node,
          visited: false,
        })
    };

    fn get_remaining_items(node_list: &mut Vec<Node>, counter: i32, start_node_name: String) -> Option<&mut Node>{
      if counter == 0 {
        return node_list.iter_mut().find(|x| x.name == start_node_name);
      }

      return node_list.iter_mut().filter(|x| x.index == 0).next();
    }

    while let Some(root_node) = get_remaining_items(&mut node_list, counter, start_node_name.to_owned()) {
      counter += 1;
      root_node.index = counter;
      root_node.visited = true;

      queue.push_back(root_node.clone());

      while queue.len() > 0 {
        counter += 1;
        let current_node = queue.pop_front().unwrap();
        let current_node_neighbors = self.adjacency_list.iter().find(|&x| x.node == current_node.name).unwrap().list.borrow();

        print!("{} ", current_node.name);

        // for all node around
        for neighbor in current_node_neighbors {
          let neighbor_node = node_list.iter_mut().find(|x| x.name == *neighbor).unwrap();
          
          if !neighbor_node.visited {
            neighbor_node.visited = true;
            neighbor_node.index = counter;

            queue.push_back(neighbor_node.clone());
          }
        }
      }
    }

  }

  fn get_node_position_by_name(&mut self, name: &str) -> Option<usize> {
    self.adjacency_list.iter().position(|x| x.node == name)
  }
}