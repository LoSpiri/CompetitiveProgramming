pub struct SegmentTree {
    pub tree: Vec<i32>,
    pub lazy: Vec<i32>,
    length: usize
}

impl SegmentTree {
    pub fn new(input: &[i32]) -> SegmentTree {
        let n = input.len();
        let next_pow_of_two = (2 * n - 1).next_power_of_two();
        let segment_tree = vec![i32::MIN; 2 * next_pow_of_two - 1];
        let lazy = vec![0; 2 * next_pow_of_two - 1];

        let mut tree = SegmentTree { tree: segment_tree, lazy, length: n };

        tree.construct_max_segment_tree(input, 0, n - 1, 0);

        tree
    }


    pub fn update_segment_tree_range_lazy(&mut self, start_range: usize, end_range: usize, delta: i32) {
        self.update_segment_tree_range_lazy_helper(start_range, end_range, delta, 0, self.length - 1, 0);
    }

    pub fn range_maximum_query_lazy(&mut self, qlow: usize, qhigh: usize) -> i32 {
        self.range_maximum_query_lazy_helper(qlow, qhigh, 0, self.length - 1, 0)
    }

    fn construct_max_segment_tree(&mut self, input: &[i32], low: usize, high: usize, pos: usize) {
        if low == high {
            self.tree[pos] = input[low];
            return;
        }

        let mid = (low + high) / 2;
        self.construct_max_segment_tree(input, low, mid, 2 * pos + 1);
        self.construct_max_segment_tree(input, mid + 1, high, 2 * pos + 2);
        self.tree[pos] = self.tree[2 * pos + 1].max(self.tree[2 * pos + 2]);
    }

    fn update_segment_tree_range_lazy_helper(&mut self, start_range: usize, end_range: usize, delta: i32, low: usize, high: usize, pos: usize) {
        // Update the lazy value for the current node if needed
        if self.lazy[pos] != 0 {
            if let Some(result) = self.tree[pos].checked_add(self.lazy[pos]) {
                self.tree[pos] = result;
            } else {
                // Handle overflow (you can choose an appropriate strategy)
                // Here, we set the value to i32::MAX, you may want to adjust it based on your requirements.
                self.tree[pos] = i32::MIN;
            }

            // If not a leaf node, propagate the lazy value to children
            if low != high {
                self.lazy[2 * pos + 1] += self.lazy[pos];
                self.lazy[2 * pos + 2] += self.lazy[pos];
            }

            // Reset the lazy value for the current node
            self.lazy[pos] = 0;
        }

        // No overlap
        if start_range > high || end_range < low {
            println!("NO OVERLAP: START - {} - END - {} - LOW - {} - HIGH - {} - POS - {}", start_range, end_range, low, high, pos);
            return;
        }

        // Total overlap
        if start_range <= low && end_range >= high {
            println!("TOTAL OVERLAP: START - {} - END - {} - LOW - {} - HIGH - {} - POS - {}", start_range, end_range, low, high, pos);
            if let Some(result) = self.tree[pos].checked_add(delta) {
                self.tree[pos] = result;
            } else {
                // Handle overflow (you can choose an appropriate strategy)
                // Here, we set the value to i32::MAX, you may want to adjust it based on your requirements.
                self.tree[pos] = i32::MIN;
            }

            // If not a leaf node, propagate the lazy value to children
            if low != high {
                self.lazy[2 * pos + 1] += delta;
                self.lazy[2 * pos + 2] += delta;
            }
            return;
        }

        println!("PARTIAL OVERLAP: START - {} - END - {} - LOW - {} - HIGH - {} - POS - {}", start_range, end_range, low, high, pos);
        // Partial overlap, update both children
        let mid = (low + high) / 2;
        self.update_segment_tree_range_lazy_helper(start_range, end_range, delta, low, mid, 2 * pos + 1);
        self.update_segment_tree_range_lazy_helper(start_range, end_range, delta, mid + 1, high, 2 * pos + 2);

        // Update the parent node based on the children
        self.tree[pos] = self.tree[2 * pos + 1].max(self.tree[2 * pos + 2]);
    }

    fn range_maximum_query_lazy_helper(&mut self, qlow: usize, qhigh: usize, low: usize, high: usize, pos: usize) -> i32 {
        // Update the lazy value for the current node if needed
        if self.lazy[pos] != 0 {
            if let Some(result) = self.tree[pos].checked_add(self.lazy[pos]) {
                self.tree[pos] = result;
            } else {
                // Handle overflow (you can choose an appropriate strategy)
                // Here, we set the value to i32::MAX, you may want to adjust it based on your requirements.
                self.tree[pos] = i32::MIN;
            }

            // If not a leaf node, propagate the lazy value to children
            if low != high {
                self.lazy[2 * pos + 1] += self.lazy[pos];
                self.lazy[2 * pos + 2] += self.lazy[pos];
            }

            // Reset the lazy value for the current node
            self.lazy[pos] = 0;
        }

        // No overlap
        if qlow > high || qhigh < low {
            return i32::MIN;
        }

        // Total overlap
        if qlow <= low && qhigh >= high {
            return self.tree[pos];
        }

        // Partial overlap, query both children
        let mid = (low + high) / 2;
        let left_child = self.range_maximum_query_lazy_helper(qlow, qhigh, low, mid, 2 * pos + 1);
        let right_child = self.range_maximum_query_lazy_helper(qlow, qhigh, mid + 1, high, 2 * pos + 2);

        // Return the minimum value from both children
        return left_child.max(right_child);
    }

}
