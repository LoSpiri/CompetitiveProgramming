use std::collections::HashMap;

impl Solution {
    pub fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
        if nums.len() < 2 {
            return false;
        }

        let modpsums = nums
            .iter()
            .scan(0, |sum, e| {
                *sum = (*sum + e) % k;
                Some(*sum)
            })
            .collect::<Vec<_>>();

        let mut map = HashMap::new();
        for (index, val) in modpsums.iter().enumerate() {
            if (*val == 0 && index != 0) || (map.contains_key(&val) && map[val] != index - 1) {
                return true;
            }
            else if !map.contains_key(&val)  {
                map.insert(val,index);
            }
        }
        false
    }
}