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
    let mut tree = FenwickTree::with_len(10);
    tree.add(0, 1);
    tree.add(1, 2);
    tree.add(2, 3);
    tree.add(3, 4);
    tree.add(4, 5);
    tree.add(5, 6);
    tree.add(6, 7);
    tree.add(7, 8);
    tree.add(8, 9);
    tree.add(9, 10);

    println!("{:?}", tree);
    println!("{}", tree.sum(9));
    println!("{}", tree.range_sum(8, 9));

    let mut arr = UpdateArray::with_len(6);
    arr.range_update(0, 5, 1);
    arr.range_update(0, 4, 2);
    arr.range_update(4, 5, 3);
    println!("{:?}", arr);
    println!("{}", arr.access(0));
    println!("{}", arr.access(1));
    println!("{}", arr.access(2));
    println!("{}", arr.access(3));
    println!("{}", arr.access(4));
    println!("{}", arr.access(5));
}