use std::collections::HashMap;

fn longest_k_good_segment(a: &[i32], k: usize) -> (usize, usize) {
    let mut left = 0;
    let mut right = 0;
    let mut freq_map = HashMap::new();
    let mut max_length = 0;
    let mut result = (0, 0);

    while right < a.len() {
        *freq_map.entry(a[right]).or_insert(0) += 1;

        while freq_map.len() > k {
            *freq_map.get_mut(&a[left]).unwrap() -= 1;
            if freq_map[&a[left]] == 0 {
                freq_map.remove(&a[left]);
            }
            left += 1;
        }

        if right - left + 1 > max_length {
            max_length = right - left + 1;
            result = (left, right);
        }
        right += 1;
    }
    result
}

fn main() {
    let a = vec![6, 5, 1, 2, 3, 2, 1, 4, 5];
    let k = 3;
    let longest_segment = longest_k_good_segment(&a, k);
    println!("Longest {}-good segment: {:?}", k, longest_segment);
}
