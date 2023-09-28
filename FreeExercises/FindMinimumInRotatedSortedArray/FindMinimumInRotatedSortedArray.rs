fn find_min(nums: Vec<i32>) -> i32 {
    let mut low = 0;
    let mut high = nums.len() - 1;

    while low < high {
        let middle = low + (high - low) / 2;

        if nums[low] > nums[high] {
            if nums[middle] >= nums[low] {
                low = middle + 1;
            }
            else {
                high = middle;
            }
        }
        else {
            high = low;
        }
    }
    nums[low]
}