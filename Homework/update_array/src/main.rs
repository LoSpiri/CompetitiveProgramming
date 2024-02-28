#[derive(Debug)]
pub struct FenwickTree {
    tree: Vec<i64>,
}

impl FenwickTree {
    pub fn with_len(n: usize) -> Self {
        Self {
            tree: vec![0; n + 1],
        }
    }

    pub fn len(&self) -> usize {
        self.tree.len() - 1
    }

    /// Indexing is 0-based, even if internally we use 1-based indexing
    pub fn add(&mut self, i: usize, delta: i64) {
        let mut i = i + 1; 
        assert!(i < self.tree.len());

        while i < self.tree.len() {
            self.tree[i] += delta;
            i = Self::next_sibling(i);
        }
    }

    /// Indexing is 0-based, even if internally we use 1-based indexing
    pub fn sum(&self, i: usize) -> i64 {
        let mut i = i + 1;  

        assert!(i < self.tree.len());
        let mut sum = 0;
        while i != 0 {
            sum += self.tree[i];
            i = Self::parent(i);
        }

        sum
    }

    pub fn range_sum(&self, l: usize, r: usize) -> i64 {
        self.sum(r) - if l == 0 { 0 } else { self.sum(l - 1) }
    }

    fn isolate_trailing_one(i: usize) -> usize {
        if i == 0 {
            0
        } else {
            1 << i.trailing_zeros()
        }
    }

    fn parent(i: usize) -> usize {
        i - Self::isolate_trailing_one(i)
    }

    fn next_sibling(i: usize) -> usize {
        i + Self::isolate_trailing_one(i)
    }
}

#[derive(Debug)]
struct UpdateArray {
    ft: FenwickTree,
}

impl UpdateArray {
    pub fn with_len(n: usize) -> Self {
        Self {
            ft: FenwickTree::with_len(n),
        }
    }

    pub fn len(&self) -> usize {
        self.ft.len()
    }

    pub fn access(&self, i: usize) -> i64 {
        self.ft.sum(i)
    }

    pub fn range_update(&mut self, l: usize, r: usize, v: i64) {
        assert!(l <= r);
        assert!(r < self.ft.len());

        self.ft.add(l, v);
        if r + 1 < self.ft.len() {
            self.ft.add(r + 1, -v);
        }
    }
}

fn main() {
    let test_cases = vec![
        (5, vec![(0, 1, 7), (2, 4, 6), (1, 3, 2)]),
    ];

    for (n, updates) in test_cases {
        let mut tree = UpdateArray::with_len(n);

        for (l, r, val) in updates {
            tree.range_update(l, r, val);
            println!("{:?}", tree.ft.tree);
        }

        let queries = vec![0, 3, 4];
        for index in queries {
            println!("{}", tree.access(index));
        }
    }
}