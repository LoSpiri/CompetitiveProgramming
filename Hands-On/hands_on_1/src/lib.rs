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
        self.rec_validate_max_heap(Some(0), 0, u32::MAX)
    }

    fn rec_validate_max_heap(
        &self,
        node_id: Option<usize>,
        index: usize,
        parent_value: u32,
    ) -> bool {
        if let Some(id) = node_id {
            let total_nodes = self.nodes.len();
            assert!(id < total_nodes, "Node id is out of range");
            let node = &self.nodes[id];
            let current_value = node.key;

            if parent_value < current_value || index >= total_nodes {
                return false;
            }

            let left_valid = self.rec_validate_max_heap(node.id_left, 2 * index + 1, current_value);
            let right_valid =
                self.rec_validate_max_heap(node.id_right, 2 * index + 2, current_value);

            return left_valid && right_valid;
        }
        true
    }
}

// -----------------------------  TEST START HERE  --------------------------------------------

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_sum() {
        let mut tree = Tree::with_root(10);

        assert_eq!(tree.sum(), 10);

        tree.add_node(0, 5, true); // id 1
        tree.add_node(0, 22, false); // id 2

        assert_eq!(tree.sum(), 37);

        tree.add_node(1, 7, false); // id 3
        tree.add_node(2, 20, true); // id 4

        assert_eq!(tree.sum(), 64);
    }

    // -------------------------------------------------------------------------

    #[test]
    fn test_valid_bst() {
        // Creare un albero binario di ricerca valido e testare se viene accettato.
        let mut tree = Tree::with_root(2);
        let left_child_id = tree.add_node(0, 1, true);
        let right_child_id = tree.add_node(0, 3, false);
        assert!(tree.validate_bst());
    }

    #[test]
    fn test_valid_bst_all_right() {
        let mut tree = Tree::with_root(1);
        let left_child_id = tree.add_node(0, 2, false);
        let left_child_id2 = tree.add_node(left_child_id, 3, false);
        let left_child_id3 = tree.add_node(left_child_id2, 4, false);
        let left_child_id4 = tree.add_node(left_child_id3, 5, false);
        assert!(tree.validate_bst());
    }

    #[test]
    fn test_invalid_bst() {
        // Creare un albero binario di ricerca non valido e testare se viene rifiutato.
        let mut tree = Tree::with_root(2);
        let left_child_id = tree.add_node(0, 3, true); // Nodo errato
        let right_child_id = tree.add_node(0, 1, false); // Nodo errato
        assert!(!tree.validate_bst());
    }

    #[test]
    fn test_single_node_bst() {
        // Testare un albero con un solo nodo, che dovrebbe essere valido.
        let mut tree = Tree::with_root(1);
        assert!(tree.validate_bst());
    }

    #[test]
    fn test_single_node_tree_edge_key() {
        // Testare un albero con un solo nodo, che dovrebbe essere valido.
        let mut tree = Tree::with_root(u32::MAX);
        assert!(tree.validate_bst());
    }

    #[test]
    fn test_max_values_bst() {
        // Testare un albero con nodi contenenti il valore massimo possibile, che dovrebbe essere valido.
        let mut tree = Tree::with_root(u32::MAX);
        let left_child_id = tree.add_node(0, u32::MIN, true);
        let right_child_id = tree.add_node(0, u32::MAX, false);
        assert!(tree.validate_bst());
    }

    // -------------------------------------------------------------------------

    #[test]
    fn test_balanced_tree() {
        // Creare un albero bilanciato e verificare che venga accettato.
        let mut tree = Tree::with_root(1);
        let left_child_id = tree.add_node(0, 2, true);
        let right_child_id = tree.add_node(0, 3, false);
        let left_left_child_id = tree.add_node(left_child_id, 4, true);
        let left_right_child_id = tree.add_node(left_child_id, 5, false);
        assert!(tree.validate_balanced());
    }

    #[test]
    fn test_unbalanced_tree() {
        // Creare un albero non bilanciato e verificare che venga rifiutato.
        let mut tree = Tree::with_root(1);
        let left_child_id = tree.add_node(0, 2, true);
        let right_child_id = tree.add_node(0, 3, false);
        let left_left_child_id = tree.add_node(left_child_id, 4, true);
        let left_left_left_child_id = tree.add_node(left_left_child_id, 5, true);
        assert!(!tree.validate_balanced());
    }

    #[test]
    fn test_single_node_balanced() {
        // Verifica che un albero con un solo nodo venga accettato come bilanciato.
        let mut tree = Tree::with_root(1);
        assert!(tree.validate_balanced());
    }

    #[test]
    fn test_single_child_balanced() {
        // Verifica che un albero con un solo nodo venga accettato come bilanciato.
        let mut tree = Tree::with_root(1);
        let left_child_id = tree.add_node(0, 2, true);
        assert!(tree.validate_balanced());
    }

    // -------------------------------------------------------------------------

    #[test]
    fn test_valid_max_heap() {
        // Creare un max-heap valido e testare se viene accettato.
        let mut tree = Tree::with_root(10);
        let left_child_id = tree.add_node(0, 8, true);
        let right_child_id = tree.add_node(0, 7, false);
        let left_left_id = tree.add_node(left_child_id, 5, true);
        let left_right_id = tree.add_node(left_child_id, 6, false);
        assert!(tree.validate_max_heap());
    }

    #[test]
    fn test_invalid_max_heap_not_max() {
        // Creare un max-heap non valido e testare se viene rifiutato.
        let mut tree = Tree::with_root(10);
        let left_child_id = tree.add_node(0, 15, true); // Nodo errato
        let right_child_id = tree.add_node(0, 12, false); // Nodo errato
        assert!(!tree.validate_max_heap());
    }

    #[test]
    fn test_invalid_max_heap_complete() {
        // Creare un max-heap non valido e testare se viene rifiutato.
        let mut tree = Tree::with_root(10);
        let left_child_id = tree.add_node(0, 8, true);
        assert!(tree.validate_max_heap());
    }

    #[test]
    fn test_invalid_max_heap_not_complete_1() {
        // Creare un max-heap non valido e testare se viene rifiutato.
        let mut tree = Tree::with_root(10);
        let left_child_id = tree.add_node(0, 8, false); // Nodo errato
        assert!(!tree.validate_max_heap());
    }

    #[test]
    fn test_invalid_max_heap_not_complete_2() {
        // Creare un max-heap non valido e testare se viene rifiutato.
        let mut tree = Tree::with_root(10);
        let left_child_id = tree.add_node(0, 8, true);
        let right_child_id = tree.add_node(0, 7, false);
        let left_left_id = tree.add_node(left_child_id, 5, true);
        let left_right_id = tree.add_node(left_child_id, 6, false);
        let left_left_left_id = tree.add_node(left_left_id, 3, true);
        let left_left_right_id = tree.add_node(left_left_id, 4, false);
        assert!(!tree.validate_max_heap());
    }

    #[test]
    fn test_single_node_max_heap() {
        // Testare un albero con un solo nodo, che dovrebbe essere valido come max-heap.
        let mut tree = Tree::with_root(5);
        assert!(tree.validate_max_heap());
    }

    #[test]
    fn test_max_values_max_heap() {
        // Testare un max-heap con nodi contenenti il valore massimo possibile, che dovrebbe essere valido.
        let mut tree = Tree::with_root(u32::MAX);
        let left_child_id = tree.add_node(0, u32::MAX, true);
        let right_child_id = tree.add_node(0, u32::MAX, false);
        assert!(tree.validate_max_heap());
    }
}
