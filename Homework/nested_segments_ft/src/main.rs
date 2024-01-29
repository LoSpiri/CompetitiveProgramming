use std::vec;

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

#[derive(PartialOrd, Ord, PartialEq, Eq, Debug)]
enum Event {
    Begin,
    End,
}

fn main() {
    let mut ranges = vec![(0, 7), (1, 2), (3, 6), (4, 5)];

    let mut pairs: Vec<_> = ranges
        .iter()
        .flat_map(|&(b, e)| vec![(b, Event::Begin), (e, Event::End)])
        .collect();
    pairs.sort_unstable();
    println!("pairs = {:?}", pairs);

    let mut ft = FenwickTree::with_len(8);

    for (x, event) in pairs.iter() {
        if *event == Event::End {
            ft.add(*x, 1);
        }
    }
    println!("ft = {:?}", ft);

    let mut ans: Vec<i64> = vec![];
    ranges.sort_unstable();
    println!("sorted_ranges = {:?}", ranges);

    for (_, y) in ranges.iter() {
        ans.push(ft.sum(y - 1));
        ft.add(*y, - 1);
    }
    println!("ans = {:?}", ans);
}