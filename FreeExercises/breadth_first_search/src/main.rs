use std::collections::{HashMap, HashSet, VecDeque};

struct Graph {
    adj_list: HashMap<usize, HashSet<usize>>,
}

impl Graph {
    fn new(adj_list: HashMap<usize, HashSet<usize>>) -> Self {
        Graph { adj_list }
    }

    fn bfs(&self, start: usize) -> Vec<usize> {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        let mut result = Vec::new();

        queue.push_back(start);
        visited.insert(start);

        while let Some(node) = queue.pop_front() {
            result.push(node);

            if let Some(neighbors) = self.adj_list.get(&node) {
                for &neighbor in neighbors {
                    if !visited.contains(&neighbor) {
                        visited.insert(neighbor);
                        queue.push_back(neighbor);
                    }
                }
            }
        }

        result
    }
}

fn main() {
    let mut adj_list = HashMap::new();
    adj_list.insert(0, vec![1, 2].into_iter().collect());
    adj_list.insert(1, vec![0, 3, 4].into_iter().collect());
    adj_list.insert(2, vec![0, 4].into_iter().collect());
    adj_list.insert(3, vec![1].into_iter().collect());
    adj_list.insert(4, vec![1, 2, 5].into_iter().collect());
    adj_list.insert(5, vec![4].into_iter().collect());

    let graph = Graph::new(adj_list);
    let start_node = 0;

    let bfs_result = graph.bfs(start_node);

    println!("BFS traversal result: {:?}", bfs_result);
}
