// https://leetcode.com/problems/maximum-subarray/description/
// KadaneAlgorithm
// Taking advantage of the fact that contiguos subarrays should be included in the max-subarray 
// Only if they have a positive total sum
// Otherwise is better to left them out  


fn max_sub_array(nums: Vec<i32>) -> i32 {
    let mut max = -100000;
    let mut sum = 0;
    for num in &nums {
        sum += num;
        if sum > max {
            max = sum;
        }
        if(sum <= 0) {
            sum = 0;
        }
    }
    max
}