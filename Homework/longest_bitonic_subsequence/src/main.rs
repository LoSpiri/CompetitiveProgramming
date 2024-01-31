fn binary_search(tails: &Vec<i32>, target: i32, end: usize) -> usize {
    let mut start = 0;
    let mut end = end;
    while start < end {
        let mid = (start + end) / 2;
        if tails[mid] < target {
            start = mid + 1;
        } else {
            end = mid;
        }
    }
    end
}

fn longest_increasing_subsequence_bs(arr: &Vec<i32>) -> Vec<usize> {
    let mut tails = Vec::new();
    let mut lis = Vec::new();
    for &num in arr {
        let pos = binary_search(&tails, num, tails.len());
        if pos == tails.len() {
            tails.push(num);
        } else {
            tails[pos] = num;
        }
        lis.push(tails.len());
    }
    lis
}

fn longest_bitonic_subsequence(arr: &Vec<i32>) -> usize {
    let n = arr.len();
    if n <= 1 {
        return n;
    }

    let lis_forward = longest_increasing_subsequence_bs(arr);
    let mut reversed_arr = arr.clone();
    reversed_arr.reverse();
    let lis_backward = longest_increasing_subsequence_bs(&reversed_arr);

    let mut max_len = 0;
    for i in 0..n {
        let current_len = lis_forward[i] + lis_backward[n - i - 1] - 1;
        max_len = max_len.max(current_len);
    }
    max_len
}

fn main() {
    let arr = vec![1, 11, 2, 10, 4, 5, 2, 1];
    println!("Length of the longest bitonic subsequence: {}", longest_bitonic_subsequence(&arr));
}
