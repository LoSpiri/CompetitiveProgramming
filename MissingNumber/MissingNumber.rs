//https://leetcode.com/problems/missing-number/

fn missing_number(nums: Vec<i32>) -> i32 {
    let len = nums.len();
    let sum_usize = (len * (len + 1)) / 2;
    let sum_i32: i32 = sum_usize as i32;
    let mut actual_sum = 0;
    for num in nums {
        actual_sum += num;
    }
    return sum_i32 - actual_sum;
}