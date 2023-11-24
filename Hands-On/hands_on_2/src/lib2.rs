pub struct SegmentTree2 {
    pub tree: Vec<i32>,
    pub lazy: Vec<i32>,
    length: usize
}

impl SegmentTree2 {
    pub fn new(input: &Vec<(usize, usize)>) -> SegmentTree2 {
        let n = input.len();
        let next_pow_of_two = (2 * n - 1).next_power_of_two();
        let segment_tree = vec![0; 2 * next_pow_of_two - 1];
        let lazy = vec![0; 2 * next_pow_of_two - 1];

        let mut tree = SegmentTree2 { tree: segment_tree, lazy, length: n };

        for &(first, second) in input.iter() {
            tree.update_segment_tree_range_lazy_2(first, second);
        }

        tree
    }
    
    pub fn update_segment_tree_range_lazy_2(&mut self, start_range: usize, end_range: usize) {
        self.update_segment_tree_range_lazy_helper(start_range, end_range, 0, self.length - 1, 0);
    }

    pub fn range_maximum_query_lazy_array(&mut self, input: &Vec<(usize, usize, usize)>) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();
        for &(first, second, third) in input.iter() {
            let out = self.range_maximum_query_lazy(first, second, third as i32);
            if out > 0 {
                result.push(1);
            } else {
                result.push(0);
            }
        }
        return result;
    }

    pub fn range_maximum_query_lazy(&mut self, qlow: usize, qhigh: usize, key: i32) -> i32 {
        self.range_maximum_query_lazy_helper(qlow, qhigh, 0, self.length - 1, 0, key)
    }

    fn update_segment_tree_range_lazy_helper(&mut self, start_range: usize, end_range: usize, low: usize, high: usize, pos: usize) {
        // Update the lazy value for the current node if needed
        if self.lazy[pos] != 0 {
            if let Some(result) = self.tree[pos].checked_add(self.lazy[pos]) {
                self.tree[pos] = result;
            } else {
                self.tree[pos] = i32::MAX;
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
            // println!("NO OVERLAP: START - {} - END - {} - LOW - {} - HIGH - {} - POS - {}", start_range, end_range, low, high, pos);
            return;
        }

        // Total overlap
        if start_range <= low && end_range >= high {
            // println!("TOTAL OVERLAP: START - {} - END - {} - LOW - {} - HIGH - {} - POS - {}", start_range, end_range, low, high, pos);
            self.tree[pos] += 1;

            // If not a leaf node, propagate the lazy value to children
            if low != high {
                self.lazy[2 * pos + 1] += 1;
                self.lazy[2 * pos + 2] += 1;
            }
            return;
        }

        // println!("PARTIAL OVERLAP: START - {} - END - {} - LOW - {} - HIGH - {} - POS - {}", start_range, end_range, low, high, pos);
        // Partial overlap, update both children
        let mid = (low + high) / 2;
        self.update_segment_tree_range_lazy_helper(start_range, end_range, low, mid, 2 * pos + 1);
        self.update_segment_tree_range_lazy_helper(start_range, end_range, mid + 1, high, 2 * pos + 2);

        // Update the parent node based on the children
        self.tree[pos] = self.tree[2 * pos + 1].max(self.tree[2 * pos + 2]);
        // println!("Updated parent at pos: {} with value: {}", pos, self.tree[pos])
    }

    fn range_maximum_query_lazy_helper(&mut self, qlow: usize, qhigh: usize, low: usize, high: usize, pos: usize, key: i32) -> i32 {
        // Update the lazy value for the current node if needed
        if self.lazy[pos] != 0 {
            if let Some(result) = self.tree[pos].checked_add(self.lazy[pos]) {
                self.tree[pos] = result;
            } else {
                // Handle overflow (you can choose an appropriate strategy)
                // Here, we set the value to i32::MAX, you may want to adjust it based on your requirements.
                self.tree[pos] = i32::MAX;
            }

            // If not a leaf node, propagate the lazy value to children
            if low != high {
                self.lazy[2 * pos + 1] += self.lazy[pos];
                self.lazy[2 * pos + 2] += self.lazy[pos];
            }

            // Reset the lazy value for the current node
            self.lazy[pos] = 0;
        }
        
        // In case the key is greater than the max number of intersection of segments
        if self.tree[pos] < key {
            return 0;
        }

        // No overlap
        if qlow > high || qhigh < low {
            // println!("NO OVERLAP: START - {} - END - {} - LOW - {} - HIGH - {} - POS - {}", qlow, qhigh, low, high, pos);
            return 0;
        }

        // Total overlap
        if qlow <= low && qhigh >= high {
            // println!("TOTAL OVERLAP: START - {} - END - {} - LOW - {} - HIGH - {} - POS - {}", qlow, qhigh, low, high, pos);
            if self.tree[pos] == key {
                return 1;
            }
            if low == high {
                return 0;
            }
        }

        // println!("PARTIAL OVERLAP: START - {} - END - {} - LOW - {} - HIGH - {} - POS - {}", qlow, qhigh, low, high, pos);
        // Partial overlap, query both children
        let mid = (low + high) / 2;
        let left_child = self.range_maximum_query_lazy_helper(qlow, qhigh, low, mid, 2 * pos + 1, key);
        let right_child = self.range_maximum_query_lazy_helper(qlow, qhigh, mid + 1, high, 2 * pos + 2, key);

        // Return the minimum value from both children
        return left_child + right_child;
    }

}
