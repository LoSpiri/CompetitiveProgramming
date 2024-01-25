use std::collections::HashMap;

pub fn mos(a: &[usize], queries: &[(usize, usize)]) -> Vec<usize> {
    let mut sorted_queries: Vec<_> = queries.iter().cloned().collect();
    let mut permutation: Vec<usize> = (0..queries.len()).collect();

    let sqrt_n = ((a.len() as f64).sqrt() + 1.0) as usize;
    sorted_queries.sort_by_key(|&(l, r)| (l / sqrt_n, r));
    permutation.sort_by_key(|&i| (queries[i].0 / sqrt_n, queries[i].1));

    let powers = calculate_powers(a, &sorted_queries);

    let mut permuted_powers = vec![0; powers.len()];
    for (i, power) in permutation.into_iter().zip(powers) {
        permuted_powers[i] = power;
    }

    permuted_powers
}

fn calculate_powers(a: &[usize], queries: &[(usize, usize)]) -> Vec<usize> {
    let mut counters: HashMap<usize, usize> = HashMap::new();
    let mut answers = Vec::with_capacity(queries.len());

    let mut cur_l = 0;
    let mut cur_r = 0;
    let mut answer = 0;

    for &(l, r) in queries {
        let mut add = |i| {
            let count = counters.entry(a[i]).or_insert(0);
            *count += 1;
            answer += *count * *count * a[i] - (*count - 1) * (*count - 1) * a[i];
            // println!("add: {:?}, {:?}, {:?}, {:?}", i, a[i], count, answer)
        };

        while cur_l > l {
            cur_l -= 1;
            add(cur_l);
        }

        while cur_r <= r {
            add(cur_r);
            cur_r += 1;
        }

        let mut remove = |i| {
            let count = counters.entry(a[i]).or_insert(0);
            answer -= *count * *count * a[i] - (*count - 1) * (*count - 1) * a[i];
            *count -= 1;
            // println!("remove: {:?}, {:?}, {:?}, {:?}", i, a[i], count, answer)
        };

        while cur_l < l {
            remove(cur_l);
            cur_l += 1;
        }

        while cur_r > r + 1 {
            cur_r -= 1;
            remove(cur_r);
        }

        answers.push(answer);
    }

    answers
}

fn main() {
    let a1 = vec![1, 2, 1];
    let queries1 = vec![(0, 1), (0, 2)];

    let powers1 = mos(&a1, &queries1);
    println!("{:?}", powers1); // Output: [3, 6]

    let a2 = vec![1, 1, 2, 2, 1, 3, 1, 1];
    let queries2 = vec![(1, 6), (0, 5), (1, 6)];

    let powers2 = mos(&a2, &queries2);
    println!("{:?}", powers2); // Output: [20, 20, 20]
}
