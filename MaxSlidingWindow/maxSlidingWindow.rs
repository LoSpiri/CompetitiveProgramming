// https://leetcode.com/problems/sliding-window-maximum/
// The trick is using a deque
// You keep the deque with the front containing the biggest number
// You do that by
// Clearing the front if its index is too low (out of window)
// Loop popping from the back all that are less than the new one (wont ever need them again)
// Pushing the new number to the back
// Everytime you push the front to the return array

use std::collections::VecDeque;

fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let n = nums.len();
    let k = k as usize;

    let mut q: VecDeque<usize> = VecDeque::new();
    let mut maxs: Vec<i32> = Vec::with_capacity(n - k + 1);

    for i in 0..k {
        while (!q.is_empty()) && nums[i] > nums[*q.back().unwrap()] {
            q.pop_back();
        }
        q.push_back(i);
    }
    maxs.push(nums[*q.front().unwrap()]);

    for i in k..n {
        while !q.is_empty() && q.front().unwrap() + k <= i {
            // more idiomatic while let Some(&(p,_)) = q.front()
            q.pop_front();
        }

        while (!q.is_empty()) && nums[i] > nums[*q.back().unwrap()] {
            q.pop_back();
        }
        q.push_back(i);
        maxs.push(nums[*q.front().unwrap()]);
    }
    maxs
}