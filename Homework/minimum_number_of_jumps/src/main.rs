
fn min_jumps(arr: &[i32]) -> i32 {
    let n = arr.len();
    if n == 1 {
        return 0;
    }
    if arr[0] == 0 {
        return -1;
    }
    
    let mut jumps = 0;
    let mut max1 = 0;
    let mut range = 0;
    for i in 0..n {
        max1 = max1.max(i as i32 + arr[i]);
        if range == i {
            range = max1 as usize;
            jumps += 1;
            if range >= n - 1 {
                return jumps;
            }
        }
    }
    -1
}

fn main() {
    let arr = [1, 3, 5, 8, 9, 2, 6, 7, 6, 8, 9];
    println!("Minimum jumps needed: {}", min_jumps(&arr));
}
