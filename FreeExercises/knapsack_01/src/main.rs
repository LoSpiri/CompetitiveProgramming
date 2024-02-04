use std::cmp;

fn knapsack(max_size: usize, items: &[(usize, usize)]) -> usize {
    let n = items.len();
    let mut dp = vec![vec![0; max_size + 1]; n + 1];

    for i in 0..=n {
        for size in 0..=max_size {
            if i == 0 || size == 0 {
                dp[i][size] = 0;
            } else if items[i - 1].0 <= size {
                dp[i][size] = cmp::max(
                    dp[i - 1][size],
                    items[i - 1].1 + dp[i - 1][size - items[i - 1].0],
                );
            } else {
                dp[i][size] = dp[i - 1][size];
            }
        }
    }

    dp[n][max_size]
}

fn main() {
    let input = vec![(1, 8), (2, 4), (3, 0), (2, 5), (2, 3)];
    let max_size = 4;
    let _n = 5;

    let result = knapsack(max_size, &input);
    println!("{}", result);
}