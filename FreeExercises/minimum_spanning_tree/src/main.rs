use std::collections::HashMap;

struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(size: usize) -> Self {
        let mut parent = Vec::with_capacity(size);
        let mut rank = vec![0; size];
        for i in 0..size {
            parent.push(i);
        }
        println!("{:?}", parent);
        println!("{:?}", rank);

        UnionFind { parent, rank }
    }

    fn find(&mut self, mut x: usize) -> usize {
        while self.parent[x] != x {
            self.parent[x] = self.parent[self.parent[x]];
            x = self.parent[x];
        }
        x
    }

    fn union(&mut self, x: usize, y: usize) {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x == root_y {
            return; 
        }

        if self.rank[root_x] < self.rank[root_y] {
            self.parent[root_x] = root_y;
        } else if self.rank[root_x] > self.rank[root_y] {
            self.parent[root_y] = root_x;
        } else {
            self.parent[root_y] = root_x;
            self.rank[root_x] += 1;
        }
    }
}

fn kruskal_mst(n: usize, edges: &[(usize, usize, usize)]) -> usize {
    let mut uf = UnionFind::new(n);
    let mut total_weight = 0;
    let mut sorted_edges = edges.to_vec();
    sorted_edges.sort_by_key(|&(_, _, weight)| weight);

    for &(u, v, weight) in &sorted_edges {
        if uf.find(u) != uf.find(v) {
            uf.union(u, v);
            total_weight += weight;
        }
        println!("{:?}", uf.parent);
        println!("{:?}", uf.rank);
        println!("----------------");
    }

    total_weight
}

fn main() {
    let input = vec![
        (1, 2, 10),
        (2, 3, 15),
        (1, 3, 5),
        (4, 2, 2),
        (4, 3, 40),
    ];
    let n_edges = 5;
    let mst_weight = kruskal_mst(n_edges, &input);
    println!("{}", mst_weight); // Output: 17
}
