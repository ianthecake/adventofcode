use std::{
    cmp::Ordering,
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn file_to_values(filename: impl AsRef<Path>) -> Vec<Vec<i32>> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);

    let mut lines: Vec<Vec<i32>> = Vec::new();

    for line in buf.lines() {
        let line = line.unwrap();
        let line_splitted: Vec<&str> = line.split_whitespace().collect();
        let mut extracted_values: Vec<i32> = Vec::new();

        for str_val in line_splitted {
            let value: i32 = str_val.parse().unwrap();
            extracted_values.push(value);
        }
        lines.push(extracted_values);
    }
    lines
}

fn check_safe(line_values: Vec<i32>) -> bool {
    let mut is_safe: bool = false;

    enum ValueOrder {
        Increasing,
        Decreasing,
    }

    let order = match line_values[1].cmp(&line_values[0]) {
        Ordering::Less => ValueOrder::Decreasing,
        Ordering::Greater => ValueOrder::Increasing,
        Ordering::Equal => return is_safe,
    };

    for i in 0..line_values.len() - 1 {
        let val = line_values[i];
        let next_val = line_values[i + 1];

        // check for difference of min 1 and max 3
        let abs_diff = (val - next_val).abs();
        if !(1..=3).contains(&abs_diff) {
            return is_safe;
        }

        // check for consistent in- or decreasing values
        match order {
            ValueOrder::Increasing => {
                println!("order: Increasing, LINE: {:?}", line_values);
                if next_val < val {
                    println!("failed");
                    return is_safe;
                }
            }

            ValueOrder::Decreasing => {
                println!("order: Decreasing, LINE: {:?}", line_values);
                if next_val > val {
                    println!("failed");
                    return is_safe;
                }
            }
        }
    }
    is_safe = true;
    is_safe
}

fn main() {
    let file = "input.txt";

    let mut safe_reports_count = 0;
    let line_values = file_to_values(file);

    for line in line_values {
        if check_safe(line) {
            safe_reports_count += 1;
        }
    }

    println!("SAFE REPORTS IN FILE: {:?}", safe_reports_count);
}
