fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut low = 0;
    let mut high = nums.len() - 1;

    if high < 0 {
        return vec![-1,-1];
    }

    while low <= high && high < nums.len() {
        let middle = low + (high-low)/2;

        if nums[middle] > target {
            high = middle - 1;
        }
        else if nums[middle] < target {
            low = middle + 1;
        }
        else {
            if nums[low] < target {
                low += 1;
            } 
            if nums[high] > target {
                high -= 1;
            }
            if nums[low] == nums[high] {
                return vec![low as i32, high as i32];
            }
        } 
    }
    vec![-1,-1]
}