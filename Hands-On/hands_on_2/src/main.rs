use hands_on_2::SegmentTree;

fn main() {
    let mut st_lazy = SegmentTree::new(&[-1, 3, 4, 0, 2, 1]);

    println!("{:?}", st_lazy.tree);
    println!("{:?}", st_lazy.lazy);

    st_lazy.update_segment_tree_range_lazy(0, 3, 3);
    // st_lazy.update_segment_tree_range_lazy(0, 3, 1);
    // st_lazy.update_segment_tree_range_lazy(0, 0, 2);

    println!("{:?}", st_lazy.tree);
    println!("{:?}", st_lazy.lazy);
    assert_eq!(7, st_lazy.range_maximum_query_lazy(2, 5));
    println!("IO SONO ERMELLINO")
}