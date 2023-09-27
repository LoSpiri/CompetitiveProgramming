fn find_peak_element(nums: Vec<i32>) -> i32 {
    let mut low: i32 = 0;
    let mut high: i32 = nums.len() as i32 - 1;

    while low < high - 1 {
        let middle = (low + (high - low) / 2) as usize;

        if nums[middle] > nums[middle + 1] && nums[middle] > nums[middle - 1] {
            return middle as i32;
        }
        else if nums[middle] < nums[middle + 1] {
            low = middle as i32 + 1;
        }
        else {
            high = middle as i32;
        }
    }
    if nums[low as usize] > nums[high as usize] {
        low as i32
    }
    else {
        high as i32
    }
}