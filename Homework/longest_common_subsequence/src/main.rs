fn lcs_length(x: &str, y: &str) -> usize {
    let m = x.len();
    let n = y.len();
    
    let mut dp = vec![vec![0; n + 1]; m + 1];
    
    for i in 1..=m {
        for j in 1..=n {
            if x.chars().nth(i - 1) == y.chars().nth(j - 1) {
                dp[i][j] = dp[i - 1][j - 1] + 1;
            } else {
                dp[i][j] = std::cmp::max(dp[i - 1][j], dp[i][j - 1]);
            }
        }
    }
    
    dp[m][n]
}

fn main() {
    let x = "AGGTAB";
    let y = "GXTXAYB";
    println!("Length of LCS: {}", lcs_length(x, y));

    let x = "ABCDGH";
    let y = "AEDFHR";
    println!("Length of LCS: {}", lcs_length(x, y));
}