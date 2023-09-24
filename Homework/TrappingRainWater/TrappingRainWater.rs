// https://leetcode.com/problems/trapping-rain-water/
// The trick here is still using a deque
// And keeping the highest on the front
// You do that by
// Looping through until you find one greater or equal to the front
// You then add water by subtracting the heights found from the front
// This clears the deque and leaves only the new number just found
// After looping through the array
// You transform the deque in array, reversing it
// And repeat the process
// This is because when the highest height is found
// The check that clears and counts all the water found wont proc
// THIS PROCESS CAN BE SURELY IMPROVED (Maybe saving in the deque the height differences)
// But Leetcode runtime and memory scorings are still really good


fn trap(height: Vec<i32>) -> i32 {
    let mut total_water = 0;
    let n = height.len() as usize;

    let mut q: VecDeque<i32> = VecDeque::new();

    for i in 0..n {
        while !q.is_empty() && height[i] >= *q.front().unwrap() {
            let front = *q.front().unwrap();
            // faccio i conti
            total_water += front - q.pop_back().unwrap();
        }
        q.push_back(height[i]);
    }

    let mut deque_to_array: Vec<i32> = Vec::new();
    while (!q.is_empty()) {
        deque_to_array.push(q.pop_back().unwrap());
    }

    let n = deque_to_array.len() as usize;
    for i in 0..n {
        while !q.is_empty() && deque_to_array[i] >= *q.front().unwrap() {
            let front = *q.front().unwrap();
            // faccio i conti
            total_water += front - q.pop_back().unwrap();
        }
        q.push_back(deque_to_array[i]);
    }
    total_water
}