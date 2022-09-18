use std::{collections::{LinkedList, VecDeque}, borrow::Borrow};

#[derive(Clone)]
pub struct NodeBfs {
  name: &'static str,
  // parent_name: &'static str,
  // level: i32,
  index: i32,
  visited: bool,
}

#[derive(Clone)]
pub struct NodeDfs {
  name: &'static str,
  discovery_time: i32,
  // finish_time: i32,
  // parent_name: &'static str,
  visited: bool,
}

pub struct Adjacency {
  node: &'static str,
  list: LinkedList<&'static str>
}

pub struct Graph {
  pub(crate) adjacency_list: Vec<Adjacency>,
  pub(crate) is_directed: bool
}

impl Graph {
  pub fn new(is_directed: bool) -> Self {
    let adjacency_list = Vec::new();

    Self { adjacency_list, is_directed }
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

    if !self.is_directed {
      // add reverse edge for undirected graphs
      let to_index = self.get_node_position_by_name(to).unwrap();
      self.adjacency_list[to_index].list.push_back(from);
    }

    self.adjacency_list[node_from_index].list.push_back(to);
  }

  pub fn add_node(&mut self, name: &'static str) {
    // check if node already exists
    let search_result = self.adjacency_list.iter().any(|x| x.node == name );

    if search_result {
      println!("A node with same name of {} already exists! ❌", name);
    }

    let adj = Adjacency {
      node: name,
      list: LinkedList::new()
    };

    self.adjacency_list.push(adj);
  }

  pub fn bfs(&mut self, start_node_name: &str) {
    self.check_is_valid_node(start_node_name);

    // initializations
    let mut counter = 0;
    let mut queue: VecDeque<NodeBfs> = VecDeque::new();
    let mut node_list: Vec<NodeBfs> = Vec::with_capacity(self.adjacency_list.len());

    // node initialization
    for list_item in &self.adjacency_list {
      node_list.push(
        NodeBfs { 
          index: 0,
          // level: 0,
          // parent_name: "",
          name: list_item.node,
          visited: false,
        })
    };

    fn get_remaining_items(node_list: &mut Vec<NodeBfs>, counter: i32, start_node_name: String) -> Option<&mut NodeBfs>{
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
        let current_node_neighbors = self.get_node_neighbors(current_node.name);

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

  pub fn dfs(&mut self, start_node_name: &str) {
    self.check_is_valid_node(start_node_name);

    let mut counter = 0;
    let mut node_list: Vec<NodeDfs> = Vec::with_capacity(self.adjacency_list.len());
    let mut stack: Vec<&str> = Vec::new(); 

    // node initialization
    for list_item in &self.adjacency_list {
      node_list.push(
        NodeDfs {
          name: list_item.node,
          discovery_time: 0,
          // finish_time: 0,
          // parent_name: "",
          visited: false,
        })
    };

    while let Some(root_node) = get_remaining_items(&mut node_list, counter, start_node_name.to_owned()) {
      // add node to the stack
      stack.push(root_node.name);
  
      while !stack.is_empty() {
        counter += 1;

        let current_node_name = stack.pop().unwrap();
        let current_node = node_list.iter_mut().find(|x| x.name == current_node_name).unwrap();
  
        // check if the stack contain a node already visited
        if !current_node.visited {
          print!("{} ", current_node.name);
          current_node.visited = true;
          current_node.discovery_time = counter;
        }
  
        // get all adjacent nodes
        let neighbors = self.get_node_neighbors(current_node.name);
  
        for &neighbor_name in neighbors {
          let neighbor_node = node_list.iter_mut().find(|x| x.name == neighbor_name).unwrap();
  
          // push not visited adjacents to the stack
          if !neighbor_node.visited {
            stack.push(neighbor_node.name);
          }
        }
      }

    }

    fn get_remaining_items (node_list: &mut Vec<NodeDfs>, counter: i32, start_node_name: String) -> Option<&mut NodeDfs> {
      if counter == 0 {
        return node_list.iter_mut().find(|x| x.name == start_node_name);
      }

      return node_list.iter_mut().filter(|x| x.discovery_time == 0).next();
    }

  }

  fn get_node_neighbors(&mut self, node_name: &str) -> &LinkedList<&str> {
    return self.adjacency_list.iter().find(|&x| x.node == node_name).unwrap().list.borrow();
  }

  fn check_is_valid_node(&mut self, name: &str) {
    println!("Starting from vertex {:?}\n", name);

    // check if the start node is valid
    let start_node_result = self.get_node_position_by_name(name);

    if start_node_result == None {
      println!("Invalid node, you must to add an edge that connects to '{}' or add that node manually", name);

      std::process::exit(1);
    }
  }

  fn get_node_position_by_name(&mut self, name: &str) -> Option<usize> {
    self.adjacency_list.iter().position(|x| x.node == name)
  }
}