pub struct SegmentTree {
    pub tree: Vec<i32>,
    pub lazy: Vec<i32>,
    length: usize,
}

impl SegmentTree {
    pub fn new(input: &[i32]) -> SegmentTree {
        let n = input.len();
        let next_pow_of_two = (2 * n - 1).next_power_of_two();
        let segment_tree = vec![0; 2 * next_pow_of_two - 1];
        let lazy = vec![0; 2 * next_pow_of_two - 1];

        let mut tree = SegmentTree {
            tree: segment_tree,
            lazy,
            length: n,
        };

        tree.construct_max_segment_tree(input, 0, n - 1, 0);

        tree
    }

    pub fn range_maximum_query_lazy(&mut self, qlow: usize, qhigh: usize) -> i32 {
        self.range_maximum_query_lazy_helper(qlow - 1, qhigh - 1, 0, self.length - 1, 0)
    }

    pub fn update_segment_tree_range_lazy(
        &mut self,
        start_range: usize,
        end_range: usize,
        delta: i32,
    ) {
        self.update_segment_tree_range_lazy_helper(
            start_range - 1,
            end_range - 1,
            delta,
            0,
            self.length - 1,
            0,
        );
    }

    fn construct_max_segment_tree(&mut self, input: &[i32], low: usize, high: usize, pos: usize) {
        if low == high {
            self.tree[pos] = input[low];
            return;
        }

        let mid = (low + high) / 2;
        self.construct_max_segment_tree(input, low, mid, 2 * pos + 1);
        self.construct_max_segment_tree(input, mid + 1, high, 2 * pos + 2);
        self.tree[pos] = self.tree[2 * pos + 1] + (self.tree[2 * pos + 2]);
    }

    fn update_segment_tree_range_lazy_helper(
        &mut self,
        start_range: usize,
        end_range: usize,
        delta: i32,
        low: usize,
        high: usize,
        pos: usize,
    ) {
        if self.lazy[pos] != 0 {
            self.tree[pos] = self.tree[pos] + self.lazy[pos];

            if low != high {
                self.lazy[2 * pos + 1] = self.lazy[2 * pos + 1] + self.lazy[pos];
                self.lazy[2 * pos + 2] = self.lazy[2 * pos + 1] + self.lazy[pos];
            }

            self.lazy[pos] = 0;
        }

        if start_range > high || end_range < low {
            return;
        }

        if start_range <= low && end_range >= high {
            self.tree[pos] = self.tree[pos] + delta;

            if low != high {
                self.lazy[2 * pos + 1] = self.lazy[2 * pos + 1] + delta;
                self.lazy[2 * pos + 2] = self.lazy[2 * pos + 1] + delta;
            }
            return;
        }

        let mid = (low + high) / 2;
        self.update_segment_tree_range_lazy_helper(
            start_range,
            end_range,
            delta,
            low,
            mid,
            2 * pos + 1,
        );
        self.update_segment_tree_range_lazy_helper(
            start_range,
            end_range,
            delta,
            mid + 1,
            high,
            2 * pos + 2,
        );

        // Update the parent node based on the children
        self.tree[pos] = self.tree[2 * pos + 1] + self.tree[2 * pos + 2];
    }

    fn range_maximum_query_lazy_helper(
        &mut self,
        qlow: usize,
        qhigh: usize,
        low: usize,
        high: usize,
        pos: usize,
    ) -> i32 {
        if self.lazy[pos] != 0 {
            self.tree[pos] = self.tree[pos] + self.lazy[pos];

            if low != high {
                self.lazy[2 * pos + 1] = self.lazy[2 * pos + 1] + (self.lazy[pos]);
                self.lazy[2 * pos + 2] = self.lazy[2 * pos + 1] + (self.lazy[pos]);
            }

            self.lazy[pos] = 0;
        }

        if qlow > high || qhigh < low {
            return 0;
        }

        if qlow <= low && qhigh >= high {
            return self.tree[pos];
        }

        let mid = (low + high) / 2;
        let left_child = self.range_maximum_query_lazy_helper(qlow, qhigh, low, mid, 2 * pos + 1);
        let right_child = self.range_maximum_query_lazy_helper(qlow, qhigh, mid + 1, high, 2 * pos + 2);

        left_child + right_child
    }
}

fn main() {
    let input = vec![0,0,1,0,0,1,1,1];
    let mut segment_tree = SegmentTree::new(&input);

    println!(
        "Max value in the range [0, 7] is {}",
        segment_tree.range_maximum_query_lazy(1, 7)
    );

    segment_tree.update_segment_tree_range_lazy(7, 7, -1);

    println!(
        "Max value in the range [1, 2] after update is {}",
        segment_tree.range_maximum_query_lazy(1, 2)
    );

    segment_tree.update_segment_tree_range_lazy(2, 2, -1); 

    println!(
        "Max value in the range [3, 6] is {}",
        segment_tree.range_maximum_query_lazy(1, 6)
    );

    segment_tree.update_segment_tree_range_lazy(6, 6, -1);

    println!(
        "Max value in the range [4, 5] after update is {}",
        segment_tree.range_maximum_query_lazy(1, 5)
    );
}
