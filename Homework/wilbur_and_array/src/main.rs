fn wilbur_and_array(array: Vec<i32>) -> i32 {
    let mut wilbur_diff = 0;
    let mut steps = 0;
    for i in array.iter() {
        let diff = i - wilbur_diff;
        wilbur_diff += diff;
        steps += diff.abs();
    }
    steps
}

fn main() {
    let mut array = vec![1, 2, 3, 4, 5];
    let wilbur = wilbur_and_array(array);
    println!("Wilbur took {} steps", wilbur);
    
    array = vec![1, 2, 2, 1];
    let wilbur = wilbur_and_array(array);
    println!("Wilbur took {} steps", wilbur);
}
