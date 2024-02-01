use std::collections::{HashMap, HashSet, VecDeque};

struct Graph {
    adj_list: HashMap<usize, HashSet<usize>>,
}

impl Graph {
    fn new(edges: &[(usize, usize)]) -> Self {
        let mut adj_list = HashMap::new();

        for &(u, v) in edges {
            adj_list.entry(u).or_insert_with(HashSet::new).insert(v);
            adj_list.entry(v).or_insert_with(HashSet::new).insert(u);
        }

        Graph { adj_list }
    }

    fn is_bipartite(&self) -> bool {
        let mut colors = HashMap::new();

        for &node in self.adj_list.keys() {
            if !colors.contains_key(&node) && !self.bfs_coloring(node, &mut colors) {
                return false;
            }
        }

        true
    }

    fn bfs_coloring(&self, start_node: usize, colors: &mut HashMap<usize, bool>) -> bool {
        let mut queue = VecDeque::new();
        queue.push_back(start_node);
        colors.insert(start_node, true);

        while let Some(node) = queue.pop_front() {
            for &neighbor in self.adj_list.get(&node).unwrap_or(&HashSet::new()) {
                if !colors.contains_key(&neighbor) {
                    colors.insert(neighbor, !colors[&node]);
                    queue.push_back(neighbor);
                } else if colors[&neighbor] == colors[&node] {
                    return false;
                }
            }
        }

        true
    }
}

fn main() {
    // Example usage:
    let edges = vec![(0, 1), (0, 3), (1, 2), (2, 3)];
    let graph = Graph::new(&edges);

    if graph.is_bipartite() {
        println!("The graph is bipartite.");
    } else {
        println!("The graph is not bipartite.");
    }

    // Example usage:
    let edges = vec![(0, 1), (1, 2)];
    let graph = Graph::new(&edges);

    if graph.is_bipartite() {
        println!("The graph is bipartite.");
    } else {
        println!("The graph is not bipartite.");
    }

    // Example usage:
    let edges = vec![(2, 3), (0, 2), (0, 3), (3, 1)];
    let graph = Graph::new(&edges);

    if graph.is_bipartite() {
        println!("The graph is bipartite.");
    } else {
        println!("The graph is not bipartite.");
    }
}