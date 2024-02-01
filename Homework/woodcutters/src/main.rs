fn woodcutters(trees: Vec<(i32,i32)>) -> i32 {
    let mut last = trees[0].0;
    let mut count = 2;
    for i in 1..trees.len()-1 {
        if trees[i].0 - trees[i].1 > last {
            last = trees[i].0;
            count += 1;
        } else if trees[i].0 + trees[i].1 < trees[i+1].0 {
            last = trees[i].0 + trees[i].1;
            count += 1;
        } else {
            last = trees[i].0;
        }
    }
    count
}

fn main() {
    let trees = vec![(1, 2), (2, 1), (5, 10), (10, 9), (19,1)];
    println!("{}", woodcutters(trees));
}
