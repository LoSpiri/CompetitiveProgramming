use hands_on_2::SegmentTree;
use hands_on_2::SegmentTree2;

use std::fs::File;
use std::io;
use std::io::BufRead;
use std::num::ParseIntError;

fn read_output(file_path: &str) -> io::Result<Vec<i32>> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let numbers: io::Result<Vec<i32>> = reader.lines().map(|line| {
        line?.parse::<i32>().map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    }).collect();

    numbers
}

fn read_input_1(file_path: &str) -> io::Result<(Vec<i32>, Vec<(usize, usize, usize, Option<Result<usize, ParseIntError>>)>)> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);
    let mut lines = reader.lines();

    let first_line = lines.next().ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Empty file"))??;
    let mut n_m_iter = first_line
    .split_whitespace()
    .map(
        |s| s.parse::<usize>()
    );
    let _n = match n_m_iter.next() {
        Some(Ok(value)) => value,
        _ => return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid N")),
    };
    let m = match n_m_iter.next() {
        Some(Ok(value)) => value,
        _ => return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid M")),
    };

    let second_line = lines.next().ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Empty file"))??;
    let array: Vec<i32> = second_line
                    .split_whitespace()
                    .filter_map(|s| s.parse::<i32>().ok())
                    .collect();

    let mut queries = Vec::with_capacity(m as usize);

    for line in lines.take(m as usize) {
        let line = line?;
        let mut values_iter = line
        .split_whitespace()
        .map(
            |s| s.parse::<usize>()
        );

        if let Some(Ok(first)) = values_iter.next() {
            if let Some(Ok(second)) = values_iter.next() {
                if let Some(Ok(third)) = values_iter.next() {
                    let query = if let Some(fourth) = values_iter.next() {
                        (first, second, third, Some(fourth))
                    } else {
                        (first, second, third, None)
                    };
                    queries.push(query);
                }
            }
        }
    }

    Ok((array, queries))
}

fn read_input_2(file_path: &str) -> io::Result<(Vec<(usize, usize)>, Vec<(usize, usize, usize)>)> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);
    let mut lines = reader.lines();

    let first_line = lines.next().ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Empty file"))??;
    let mut n_m_iter = first_line
    .split_whitespace()
    .map(
        |s| s.parse::<usize>()
    );
    let n = match n_m_iter.next() {
        Some(Ok(value)) => value,
        _ => return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid N")),
    };
    let m = match n_m_iter.next() {
        Some(Ok(value)) => value,
        _ => return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid M")),
    };

    let mut segments: Vec<(usize, usize)>= Vec::with_capacity(n as usize);
    let mut queries: Vec<(usize, usize, usize)> = Vec::with_capacity(m as usize);

    for line in lines.take(n as usize + m as usize) {
        let line = line?;
        let mut values_iter = line
        .split_whitespace()
        .map(
            |s| s.parse::<usize>()
        );

        if let Some(Ok(first)) = values_iter.next() {
            if let Some(Ok(second)) = values_iter.next() {
                if values_iter.clone().count() == 0 {
                    segments.push((first, second));
                } else if let Some(Ok(third)) = values_iter.next() {
                    queries.push((first, second, third));
                }
            }
        }
    }
    Ok((segments, queries))
}


fn main() {
    // -------------------------------------------------------------------------------------------------
    // TESTING PARTE 1
    // NOTE (per me):

    for i in 0..=10 {
        if let Ok((array, queries)) = read_input_1(&format!("test1/input{}.txt", i)) {
            let mut st: SegmentTree = SegmentTree::new(&array);
            let mut result_queries_1: Vec<i32> = Vec::new();
            for (query_type, start, end, key) in queries {
                if query_type == 1 {
                    result_queries_1.push(st.range_maximum_query_lazy(start, end));
                } else {
                    let parsed_key = key.and_then(|result| result.ok());
                    match parsed_key {
                        Some(parsed_key) => {
                            st.update_segment_tree_range_lazy(start, end, parsed_key as i32);
                        }
                        None => {
                            eprintln!("Should be an update query but there's no key.");
                        }
                    }
                }
            }
            if let Ok(output_1) = read_output(&format!("test1/output{}.txt", i)) {
                assert_eq!(output_1, result_queries_1);
                println!("{:?}", result_queries_1);
            }
        }
    }

    // -------------------------------------------------------------------------------------------------
    // TESTING PARTE 2 
    // NOTE (per me):
    // La query si ferma prima anche se la chiave da trovare é maggiore del valore attuale 

    for i in 0..=7 {
        if let Ok((segments, queries)) = read_input_2(&format!("test2/input{}.txt", i)) {
            let mut st_2: SegmentTree2 = SegmentTree2::new(&segments);
            let result_queries_2 = st_2.range_maximum_query_lazy_array(&queries);
    
            if let Ok(output_2) = read_output(&format!("test2/output{}.txt", i)) {
                assert_eq!(output_2, result_queries_2);
                println!("{:?}", result_queries_2);
            }
        } else {
            eprintln!("Error reading file.");
        }
    } 
    // -------------------------------------------------------------------------------------------------

    println!("IO SONO ERMELLINO")
}