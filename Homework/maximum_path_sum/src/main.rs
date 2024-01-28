use std::cmp::max;

struct Node {
    key: u32,
    id_left: Option<usize>,
    id_right: Option<usize>,
}

impl Node {
    fn new(key: u32) -> Self {
        Self {
            key,
            id_left: None,
            id_right: None,
        }
    }
}

pub struct Tree {
    nodes: Vec<Node>,
}

impl Tree {
    pub fn with_root(key: u32) -> Self {
        Self {
            nodes: vec![Node::new(key)],
        }
    }

    pub fn add_node(&mut self, parent_id: usize, key: u32, is_left: bool) -> usize {
        assert!(
            parent_id < self.nodes.len(),
            "Parent node id does not exist"
        );
        if is_left {
            assert!(
                self.nodes[parent_id].id_left.is_none(),
                "Parent node has the child already set"
            );
        } else {
            assert!(
                self.nodes[parent_id].id_right.is_none(),
                "Parent node has the right child already set"
            );
        }

        let child_id = self.nodes.len();
        self.nodes.push(Node::new(key));

        let child = if is_left {
            &mut self.nodes[parent_id].id_left
        } else {
            &mut self.nodes[parent_id].id_right
        };

        *child = Some(child_id);

        child_id
    }

    pub fn max_path_sum(&self) -> u32 {
        // THIS CODE
        // Is meant to extend the solution to the case where there are no 2 special nodes
        // Although it is not needed for the problem because the problem states that there are 2 special nodes

        let node_1_exists = self.nodes.get(1).is_some();
        let node_2_exists = self.nodes.get(2).is_some();

        let left_max = if node_1_exists {
            self.rec_max_path_sum(Some(1))
        } else {
            0
        };
        
        let right_max = if node_2_exists {
            self.rec_max_path_sum(Some(2))
        } else {
            0
        };
        left_max + right_max + self.nodes[0].key
    }

    fn rec_max_path_sum(&self, node_id: Option<usize>) -> u32 {
        if let Some(id) = node_id {
            assert!(id < self.nodes.len(), "Node id is out of range");
            let node = &self.nodes[id];

            let max_left = self.rec_max_path_sum(node.id_left);
            let max_right = self.rec_max_path_sum(node.id_right);

            return max(max_left, max_right) + node.key;
        }
        0
    }

}

fn main() {
    let mut tree = Tree::with_root(1);
    let id_1 = tree.add_node(0, 2, true);
    let id_2 = tree.add_node(0, 3, false);
    let _id_3 = tree.add_node(id_1, 4, true);
    let _id_4 = tree.add_node(id_1, 5, false);
    let _id_5 = tree.add_node(id_2, 6, true);
    let _id_6 = tree.add_node(id_2, 7, false);
    let max_sum = tree.max_path_sum();
    println!("max sum: {}", max_sum);

    let mut tree = Tree::with_root(1);
    let max_sum = tree.max_path_sum();
    println!("max sum: {}", max_sum);

    let _id_1 = tree.add_node(0, 2, true);
    let max_sum = tree.max_path_sum();
    println!("max sum: {}", max_sum);
}