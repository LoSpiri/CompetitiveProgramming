// After looking at the leetcode constraints
// The preparation step of the sweep line algo could have been modified
// And less memory could be used to store the pairs
// But if at scale
// This solution uses way less memory and is way faster
// Assuming the pairs are way less than the range length


use std::cmp;

#[derive(PartialOrd, Ord, PartialEq, Eq, Debug)]
enum Event {
    Begin,
    End,
}
impl Solution {
    pub fn is_covered(ranges: Vec<Vec<i32>>, left: i32, right: i32) -> bool {
        let mut pairs: Vec<_> = ranges
        .iter()
        .flat_map(|vec| {
            let start = vec[0];
            let end = vec[1];
            if start <= right && end >= left {
                vec![ (start, Event::Begin), (end, Event::End) ]
            } else {
                vec![]
            }   
        })
        .collect();

        if pairs.is_empty() {
            return false
        }

        pairs.sort_unstable();

        let mut flag = true;
        let mut counter = 0;
        let mut index = 1;
        let mut index_pairs = 0;
        while index <= right {
            if (index_pairs < pairs.len() && pairs[index_pairs].0 == index) {
                if pairs[index_pairs].1 == Event::Begin {
                    counter += 1;
                }
                else {
                    flag = false;
                    counter -= 1;
                }
                index_pairs += 1;
            }
            else {
                if (index >= left && counter <= 0 && flag) {
                    return false
                }
                flag = true;
                index += 1;
            }           
        }
        return true
    }
}