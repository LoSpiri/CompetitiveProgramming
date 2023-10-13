mod data_structure;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum() {
        let mut tree = data_structure::Tree::with_root(10);

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
        let mut tree = data_structure::Tree::with_root(2);
        let left_child_id = tree.add_node(0, 1, true);
        let right_child_id = tree.add_node(0, 3, false);
        assert!(tree.validate_bst());
    }

    #[test]
    fn test_valid_bst_all_right() {
        let mut tree = data_structure::Tree::with_root(1);
        let left_child_id = tree.add_node(0, 2, false);
        let left_child_id2 = tree.add_node(left_child_id, 3, false);
        let left_child_id3 = tree.add_node(left_child_id2, 4, false);
        let left_child_id4 = tree.add_node(left_child_id3, 5, false);
        assert!(tree.validate_bst());
    }

    #[test]
    fn test_invalid_bst() {
        // Creare un albero binario di ricerca non valido e testare se viene rifiutato.
        let mut tree = data_structure::Tree::with_root(2);
        let left_child_id = tree.add_node(0, 3, true); // Nodo errato
        let right_child_id = tree.add_node(0, 1, false); // Nodo errato
        assert!(!tree.validate_bst());
    }

    #[test]
    fn test_single_node_bst() {
        // Testare un albero con un solo nodo, che dovrebbe essere valido.
        let mut tree = data_structure::Tree::with_root(1);
        assert!(tree.validate_bst());
    }

    #[test]
    fn test_single_node_tree_edge_key() {
        // Testare un albero con un solo nodo, che dovrebbe essere valido.
        let mut tree = data_structure::Tree::with_root(u32::MAX);
        assert!(tree.validate_bst());
    }

    #[test]
    fn test_max_values_bst() {
        // Testare un albero con nodi contenenti il valore massimo possibile, che dovrebbe essere valido.
        let mut tree = data_structure::Tree::with_root(u32::MAX);
        let left_child_id = tree.add_node(0, u32::MIN, true);
        let right_child_id = tree.add_node(0, u32::MAX, false);
        assert!(tree.validate_bst());
    }

    // -------------------------------------------------------------------------

    #[test]
    fn test_balanced_tree() {
        // Creare un albero bilanciato e verificare che venga accettato.
        let mut tree = data_structure::Tree::with_root(1);
        let left_child_id = tree.add_node(0, 2, true);
        let right_child_id = tree.add_node(0, 3, false);
        let left_left_child_id = tree.add_node(left_child_id, 4, true);
        let left_right_child_id = tree.add_node(left_child_id, 5, false);
        assert!(tree.validate_balanced());
    }

    #[test]
    fn test_unbalanced_tree() {
        // Creare un albero non bilanciato e verificare che venga rifiutato.
        let mut tree = data_structure::Tree::with_root(1);
        let left_child_id = tree.add_node(0, 2, true);
        let right_child_id = tree.add_node(0, 3, false);
        let left_left_child_id = tree.add_node(left_child_id, 4, true);
        let left_left_left_child_id = tree.add_node(left_left_child_id, 5, true);
        assert!(!tree.validate_balanced());
    }

    #[test]
    fn test_single_node_balanced() {
        // Verifica che un albero con un solo nodo venga accettato come bilanciato.
        let mut tree = data_structure::Tree::with_root(1);
        assert!(tree.validate_balanced());
    }

    #[test]
    fn test_single_child_balanced() {
        // Verifica che un albero con un solo nodo venga accettato come bilanciato.
        let mut tree = data_structure::Tree::with_root(1);
        let left_child_id = tree.add_node(0, 2, true);
        assert!(tree.validate_balanced());
    }

    // -------------------------------------------------------------------------

    #[test]
    fn test_valid_max_heap() {
        // Creare un max-heap valido e testare se viene accettato.
        let mut tree = data_structure::Tree::with_root(10);
        let left_child_id = tree.add_node(0, 8, true);
        let right_child_id = tree.add_node(0, 7, false);
        let left_left_id = tree.add_node(left_child_id, 5, true);
        let left_right_id = tree.add_node(left_child_id, 6, false);
        assert!(tree.validate_max_heap());
    }

    #[test]
    fn test_invalid_max_heap_not_max() {
        // Creare un max-heap non valido e testare se viene rifiutato.
        let mut tree = data_structure::Tree::with_root(10);
        let left_child_id = tree.add_node(0, 15, true); // Nodo errato
        let right_child_id = tree.add_node(0, 12, false); // Nodo errato
        assert!(!tree.validate_max_heap());
    }

    #[test]
    fn test_invalid_max_heap_not_complete_1() {
        // Creare un max-heap non valido e testare se viene rifiutato.
        let mut tree = data_structure::Tree::with_root(10);
        let left_child_id = tree.add_node(0, 8, true); // Nodo errato
        assert!(tree.validate_max_heap());
    }

    #[test]
    fn test_invalid_max_heap_not_complete_2() {
        // Creare un max-heap non valido e testare se viene rifiutato.
        let mut tree = data_structure::Tree::with_root(10);
        let left_child_id = tree.add_node(0, 8, false); // Nodo errato
        assert!(!tree.validate_max_heap());
    }

    #[test]
    fn test_invalid_max_heap_not_complete_3() {
        // Creare un max-heap non valido e testare se viene rifiutato.
        let mut tree = data_structure::Tree::with_root(10);
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
        let mut tree = data_structure::Tree::with_root(5);
        assert!(tree.validate_max_heap());
    }

    #[test]
    fn test_max_values_max_heap() {
        // Testare un max-heap con nodi contenenti il valore massimo possibile, che dovrebbe essere valido.
        let mut tree = data_structure::Tree::with_root(u32::MAX);
        let left_child_id = tree.add_node(0, u32::MAX, true);
        let right_child_id = tree.add_node(0, u32::MAX, false);
        assert!(tree.validate_max_heap());
    }

    // SI SVILUPPA GLI ALGOS CONSIDERANDO POSSIBILI VALORI NON UNIQUE
    // NON SI TESTANO ALBERI VUOTI PER IMPLEMENTAZIONE
    // NON SI TESTANO CASI IN CUI IL PARENT ID É FUORI RANGE
    // NON SI TESTANO CASI IN CUI IL NODO HA GIÁ IL FIGLIO IMPOSTATO
}
