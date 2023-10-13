use std::cmp;

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

/// This a representation of a tree.
/// Every node has an implicity id, which is its position on the vector `nodes`.
/// Every node has a key and at most two children. The ids of the children are
/// stored in `id_left` and `id_right`. These ids are `None` iff the child does not exit.
impl Tree {
    pub fn with_root(key: u32) -> Self {
        Self {
            nodes: vec![Node::new(key)],
        }
    }

    /// Adds a child to the node with `parent_id` and returns the id of the new node.
    /// The new node has the specified `key`. The new node is the left child of the node `parent_id`
    /// iff `is_left` is `true`, the right child otherwise.
    ///
    /// # Panics
    /// Panics if the `parent_id` does not exist, or if the node `parent_id ` has the child already set.
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

    /// Returns the sum of all the keys in the tree
    pub fn sum(&self) -> u32 {
        self.rec_sum(Some(0))
    }

    /// A private recursive function that computes the sum of
    /// nodes in the subtree rooted at `node_id`.
    fn rec_sum(&self, node_id: Option<usize>) -> u32 {
        if let Some(id) = node_id {
            assert!(id < self.nodes.len(), "Node id is out of range");
            let node = &self.nodes[id];

            let sum_left = self.rec_sum(node.id_left);
            let sum_right = self.rec_sum(node.id_right);

            return sum_left + sum_right + node.key;
        }
        0
    }

    pub fn validate_bst(&self) -> bool {
        self.rec_validate_bst(Some(0), u32::MIN, u32::MAX)
    }

    fn rec_validate_bst(&self, node_id: Option<usize>, min_val: u32, max_val: u32) -> bool {
        if let Some(id) = node_id {
            assert!(id < self.nodes.len(), "Node id is out of range");
            let node = &self.nodes[id];
            let current_value = node.key;

            if current_value < min_val || current_value > max_val {
                return false;
            }
            let left_valid: bool = self.rec_validate_bst(node.id_left, min_val, current_value);
            let right_valid: bool = self.rec_validate_bst(node.id_right, current_value, max_val);

            return left_valid && right_valid;
        }
        true
    }

    pub fn validate_balanced(&self) -> bool {
        self.rec_validate_balanced(Some(0)) >= 0
    }

    fn rec_validate_balanced(&self, node_id: Option<usize>) -> i32 {
        if let Some(id) = node_id {
            assert!(id < self.nodes.len(), "Node id is out of range");
            let node = &self.nodes[id];

            let left = self.rec_validate_balanced(node.id_left);
            let right = self.rec_validate_balanced(node.id_right);

            if left == -1 || right == -1 || (right - left).abs() > 1 {
                return -1;
            }
            return 1 + cmp::max(left, right);
        }
        0
    }

    pub fn validate_max_heap(&self) -> bool {
        self.rec_validate_max_heap(Some(0), u32::MAX)
    }

    fn rec_validate_max_heap(&self, node_id: Option<usize>, parent_value: u32) -> bool {
        if let Some(id) = node_id {
            assert!(id < self.nodes.len(), "Node id is out of range");
            let node = &self.nodes[id];
            let current_value = node.key;

            if parent_value < current_value || node.id_left.is_none() != node.id_right.is_none() {
                return false;
            }

            let left_valid = self.rec_validate_max_heap(node.id_left, current_value);
            let right_valid = self.rec_validate_max_heap(node.id_right, current_value);

            return left_valid && right_valid;
        }
        true
    }
}
