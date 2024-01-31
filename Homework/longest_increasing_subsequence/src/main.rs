fn longest_increasing_subsequence_dp(nums: &[i32]) -> usize {
    let mut dp = vec![1; nums.len()];
    for i in 1..nums.len() {
        for j in 0..i {
            if nums[i] > nums[j] && dp[i] < dp[j] + 1 {
                dp[i] = dp[j] + 1;
            }
        }
    }
    dp.into_iter().max().unwrap_or(0)
}

fn binary_search(tails: &Vec<i32>, target: i32, end: usize) -> usize {
    let mut start = 0;
    let mut end = end;
    while start < end {
        let mid = (start + end) / 2;
        if tails[mid] < target {
            start = mid + 1;
        } else {
            end = mid;
        }
    }
    end
}

fn longest_increasing_subsequence_bs(arr: &Vec<i32>) -> usize {
    let mut tails = Vec::new();
    for &num in arr {
        let pos = binary_search(&tails, num, tails.len());

        if pos == tails.len() {
            tails.push(num);
        } else {
            tails[pos] = num;
        }
    }
    tails.len()
}

fn main() {
    let nums = [10, 22, 9, 33, 21, 50, 41, 60, 80];
    let lis_length = longest_increasing_subsequence_dp(&nums);
    println!("Longest Increasing Subsequence - DP: {}", lis_length);

    let lis = longest_increasing_subsequence_bs(&nums.to_vec());
    println!("Longest Increasing Subsequence - BS: {:?}", lis);
}