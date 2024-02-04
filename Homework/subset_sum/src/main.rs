fn subset_sum(arr: &[i32]) -> bool {
    let total_sum: i32 = arr.iter().sum();
    if total_sum % 2 != 0 {
        return false;
    }
    let half_sum = total_sum / 2;
    let n = arr.len();
    
    let mut dp = vec![vec![false; (half_sum + 1) as usize]; (n + 1) as usize];
    
    for i in 0..=n {
        dp[i][0] = true;
    }
    for i in 1..=n {
        for j in 1..=half_sum {
            if arr[i - 1] <= j {
                dp[i][j as usize] = dp[i - 1][j as usize] || dp[i - 1][(j - arr[i - 1]) as usize];
            } else {
                dp[i][j as usize] = dp[i - 1][j as usize];
            }
        }
    }

    dp[n][half_sum as usize]
}

fn main() {
    let arr = vec![1, 5, 11, 5];
    if subset_sum(&arr) {
        println!("Array CAN be partitioned into two parts with equal sum")
    } else {
        println!("Array CANNOT be partitioned into two parts with equal sum");
    }
}