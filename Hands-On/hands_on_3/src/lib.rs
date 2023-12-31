// -------------------------------------------------------------------------------------------------
// PARTE 1
pub fn holiday_planner(input: Vec<Vec<i32>>, n: usize, days: usize) -> i32 {
    let mut prefix_sums = Vec::new();
    for i in &input {
        let mut row = vec![0];
        row.extend(i.iter().scan(0, |sum, e| {
            *sum += e;
            Some(*sum)
        }));
        prefix_sums.push(row);
    }

    let mut indices_vector: Vec<Vec<usize>> = Vec::new();
    indices_vector.push(vec![0; n]);

    let mut optimum_vector: Vec<i32> = Vec::new();
    optimum_vector.push(0);

    for day in 1..=days {
        let mut max_value = i32::MIN;
        let mut indices_of_max = vec![0; n];
        for local_day in 0..day {
            // optimum[local_day] + max(pref[day-local_day] - pref[indices[local_day]])
            let local_optimum = optimum_vector[local_day];
            let mut max_local_diff = i32::MIN;
            let mut local_indices_of_max = vec![0; n];
            for row in 0..n {
                let local_diff = prefix_sums[row][day - local_day]
                    - prefix_sums[row][indices_vector[local_day][row]];
                if local_diff >= max_local_diff {
                    max_local_diff = local_diff;
                    let mut local_indices = indices_vector[local_day].clone();
                    local_indices[row] += day - local_day;
                    local_indices_of_max = local_indices;
                }
            }
            if local_optimum + max_local_diff >= max_value {
                // Favors staying in the same place longer and minimizes transportation
                max_value = local_optimum + max_local_diff;
                indices_of_max = local_indices_of_max;
            }
        }
        optimum_vector.push(max_value);
        indices_vector.push(indices_of_max);
    }
    return *optimum_vector.last().unwrap();
}

// -------------------------------------------------------------------------------------------------
// PARTE 2

pub fn max_topics(mut input: Vec<(i32, i32)>, length: usize) -> usize {
    input.sort_unstable_by_key(|it| (it.0, std::cmp::Reverse(it.1)));

    let mut max_increasing: Vec<(i32, i32)> = Vec::new();
    max_increasing.push(input[0]);

    for i in input.iter().take(length).skip(1) {
        if i.1 > max_increasing.last().unwrap().1 {
            max_increasing.push(*i);
        } else {
            let mut low = 0;
            let mut high = max_increasing.len() - 1;
            while low < high {
                let mid = low + (high - low) / 2;
                if max_increasing[mid].1 < i.1 {
                    low = mid + 1;
                } else {
                    high = mid;
                }
            }
            max_increasing[low] = *i;
        }
    }
    max_increasing.len()
}

fn ceil_index(input: &[(i32, i32)], t: &[usize], mut end: usize, s: (i32, i32)) -> isize {
    let mut start = 0;
    let mut middle;
    let len = end;
    while start <= end {
        middle = (start + end) / 2;
        if middle < len && input[t[middle]].1 < s.1 && s.1 <= input[t[middle + 1]].1 {
            return (middle + 1) as isize;
        } else if input[t[middle]].1 < s.1 {
            start = middle + 1;
        } else {
            end = middle - 1;
        }
    }
    -1
}

pub fn longest_increasing_subsequence(input: &mut Vec<(i32, i32)>) -> usize {
    let input_len = input.len();
    input.sort_unstable_by_key(|it| (it.0, std::cmp::Reverse(it.1)));
    println!("{:?}", input);

    let mut t = vec![0; input_len];
    let mut r = vec![usize::MAX; input_len];
    t[0] = 0;
    let mut len = 0;

    for i in 1..input_len {
        if input[t[0]].1 >= input[i].1 {
            t[0] = i;
        } else if input[t[len]].1 < input[i].1 {
            len += 1;
            t[len] = i;
            r[t[len]] = t[len - 1];
        } else {
            let index = ceil_index(input, &t, len, input[i]);
            t[index as usize] = i;
            r[t[index as usize]] = t[index as usize - 1];
        }
    }

    println!("Longest increasing subsequence: ");
    let mut index = t[len];
    while index != usize::MAX {
        print!("{} ", input[index].1);
        index = r[index];
    }

    len + 1
}
