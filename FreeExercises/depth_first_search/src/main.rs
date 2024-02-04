use std::collections::{HashMap, HashSet};

struct Graph {
    adj_list: HashMap<usize, HashSet<usize>>,
}

impl Graph {
    fn new(adj_list: HashMap<usize, HashSet<usize>>) -> Self {
        Graph { adj_list }
    }

    fn dfs(&self, start: usize, visited: &mut HashSet<usize>, result: &mut Vec<usize>) {
        visited.insert(start);
        result.push(start);

        if let Some(neighbors) = self.adj_list.get(&start) {
            for &neighbor in neighbors {
                if !visited.contains(&neighbor) {
                    self.dfs(neighbor, visited, result);
                }
            }
        }
    }

    fn dfs_traversal(&self, start: usize) -> Vec<usize> {
        let mut visited = HashSet::new();
        let mut result = Vec::new();
        self.dfs(start, &mut visited, &mut result);
        result
    }
}

fn main() {
    let mut adj_list = HashMap::new();
    adj_list.insert(0, vec![1, 2].into_iter().collect());    // Node 0 is connected to nodes 1 and 2
    adj_list.insert(1, vec![0, 3, 4].into_iter().collect()); // Node 1 is connected to nodes 0, 3, and 4
    adj_list.insert(2, vec![0, 4].into_iter().collect());    // Node 2 is connected to nodes 0 and 4
    adj_list.insert(3, vec![1].into_iter().collect());       // Node 3 is connected to node 1
    adj_list.insert(4, vec![1, 2, 5].into_iter().collect()); // Node 4 is connected to nodes 1, 2, and 5
    adj_list.insert(5, vec![4].into_iter().collect());       // Node 5 is connected to node 4

    let graph = Graph::new(adj_list);
    let start_node = 0; // Starting DFS from node 0

    let dfs_result = graph.dfs_traversal(start_node);

    println!("DFS traversal result: {:?}", dfs_result);
}
