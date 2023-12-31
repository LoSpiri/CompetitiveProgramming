use std::{
    fs::File,
    io::{self, BufRead},
};

use hands_on_3::{holiday_planner, longest_increasing_subsequence, max_topics};

fn read_output(file_path: &str) -> io::Result<Vec<i32>> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let numbers: io::Result<Vec<i32>> = reader
        .lines()
        .map(|line| {
            line?
                .parse::<i32>()
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
        })
        .collect();

    numbers
}

fn read_input_1(file_path: &str) -> io::Result<(Vec<Vec<i32>>, usize, usize)> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);
    let mut lines = reader.lines();

    let first_line = lines
        .next()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Empty file"))??;
    let mut n_m_iter = first_line.split_whitespace().map(|s| s.parse::<usize>());
    let n = match n_m_iter.next() {
        Some(Ok(value)) => value,
        _ => return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid N")),
    };
    let days = match n_m_iter.next() {
        Some(Ok(value)) => value,
        _ => return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid M")),
    };

    let mut input: Vec<Vec<i32>> = Vec::with_capacity(n);
    for line in lines.take(n) {
        let line = line?;
        input.push(
            line.split_whitespace()
                .filter_map(|s| s.parse::<i32>().ok())
                .collect(),
        );
    }
    Ok((input, n, days))
}

fn read_input_2(file_path: &str) -> io::Result<(Vec<(i32, i32)>, usize)> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);
    let mut lines = reader.lines();

    let first_line = lines
        .next()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Empty file"))??;
    let topics_len = first_line.trim().parse().map_err(|_| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            "Impossibile convertire la stringa in usize",
        )
    })?;

    let mut topics = Vec::with_capacity(topics_len);
    for line in lines.take(topics_len) {
        let line = line?;
        let mut values_iter = line.split_whitespace().map(|s| s.parse::<i32>());

        if let Some(Ok(first)) = values_iter.next() {
            if let Some(Ok(second)) = values_iter.next() {
                topics.push((first, second))
            }
        }
    }
    Ok((topics, topics_len))
}

fn main() {
    // -------------------------------------------------------------------------------------------------
    // TESTING PARTE 1
    for i in 0..=5 {
        if let Ok((input, n, days)) = read_input_1(&format!("test1/input{}.txt", i)) {
            let max_attractions_visitable = holiday_planner(input, n, days);

            if let Ok(output_1) = read_output(&format!("test1/output{}.txt", i)) {
                assert_eq!(output_1[0], max_attractions_visitable);
            }
        } else {
            eprintln!("Error reading file.");
        }
    }
    // -------------------------------------------------------------------------------------------------
    // TESTING PARTE 2
    for i in 0..=13 {
        if let Ok((topics, topics_len)) = read_input_2(&format!("test2/input{}.txt", i)) {
            let max_increasing_topics = max_topics(topics, topics_len);
            // let max_increasing_topics = longest_increasing_subsequence(&mut topics);

            if let Ok(output_2) = read_output(&format!("test2/output{}.txt", i)) {
                assert_eq!(output_2[0], max_increasing_topics.try_into().unwrap());
            }
        } else {
            eprintln!("Error reading file.");
        }
    }
}
