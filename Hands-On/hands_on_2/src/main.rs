use hands_on_2::SegmentTree;

use std::fs::File;
use std::io::{BufRead, BufReader};

use std::path::{PathBuf, Path};
use std::str::SplitWhitespace;

fn main() {
    
    // -----------------------------
    for i in 0..=1 {
        let file_path: PathBuf = format!("test1/input{}.txt", i).into();
        if let Ok(file) = File::open(file_path) {
            let reader = BufReader::new(file);
            let mut array_length: i32;
            let mut total_queries: i32;
            let mut counter = 0;
            let mut st: SegmentTree = SegmentTree::new(&[0]);
            let mut result_max_queries: Vec<i32> = Vec::new();
            for line in reader.lines() {
                if let Ok(line_content) = line {
                    // println!("{}", line_content);

                    let values: Result<Vec<i32>, _> = line_content
                    .split_whitespace()
                    .map(|s| s.parse::<i32>())
                    .collect();

                    match values {
                        Ok(parsed_values) => {
                            if counter == 0 {
                                array_length = parsed_values[0];
                                total_queries = parsed_values[1];
                            } else if counter == 1 {
                                st = SegmentTree::new(&parsed_values);
                                println!("{:?}", st.tree);
                            } else {
                                if parsed_values[0] == 0 {
                                    st.update_segment_tree_range_lazy(parsed_values[1] as usize - 1, parsed_values[2] as usize - 1, parsed_values[3]);
                                } else if parsed_values [0] == 1 {
                                    result_max_queries.push(st.range_maximum_query_lazy(parsed_values[1] as usize - 1, parsed_values[2] as usize - 1));
                                } else {
                                    eprintln!("Query is neither update nor max");
                                }
                            }
                        }
                        Err(err) => {
                            // Gestione dell'errore di parsing
                            eprintln!("Error parsing values: {}", err);
                        }
                    }
                    counter += 1;
                } else {
                    eprintln!("Error reading line from file");
                }
            }
            println!("{:?}", result_max_queries);
        } else {
            eprintln!("Error opening file");
        }
    }
    // -----------------------------
    
    // let mut st_lazy = SegmentTree::new(&[2, 6, 7, 0, 2, 1]);
    // println!("{:?}", st_lazy.tree);
    // println!("{:?}", st_lazy.lazy);
    // assert_eq!(7, st_lazy.range_maximum_query_lazy(2, 5));
    // st_lazy.update_segment_tree_range_lazy(2, 4, 4);
    // // st_lazy.update_segment_tree_range_lazy(0, 3, 1);
    // // st_lazy.update_segment_tree_range_lazy(0, 0, 2);
    // assert_eq!(4, st_lazy.range_maximum_query_lazy(2, 5));
    // println!("{:?}", st_lazy.tree);
    // println!("{:?}", st_lazy.lazy);
    println!("IO SONO ERMELLINO")
}