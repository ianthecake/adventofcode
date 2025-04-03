// adventofcode DAY 1-2
//
// Compute a similarity score between two lists containing numbers by
// multiplying each value from the first list with the number of occurances of that number in the
// second list and adding the results together.
//
// SOLUTION:
// The similarity score is 22962826


use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};



fn file_to_list(filename: impl AsRef<Path>) -> (Vec<i32>, Vec<i32>) {

    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);

    let mut left_list: Vec<i32> = Vec::new();
    let mut right_list: Vec<i32> = Vec::new();

    for line in buf.lines() {
        let line = line.unwrap();
        let line_splitted: Vec<&str> = line.split_whitespace().collect();

        if !line_splitted.is_empty() {
            left_list.push(line_splitted[0].parse::<i32>().unwrap());
            right_list.push(line_splitted[1].parse::<i32>().unwrap());
        }
    }
    return (left_list, right_list)
}

fn get_similarity_score(list1: Vec<i32>, list2: Vec<i32>) -> i32 {

    let mut similarity_score = 0;

    for id1 in list1 {
        let mut id_count = 0;
        for id2 in &list2 {
            if id1 == *id2 {
                id_count += 1;
            }
        }
        similarity_score += id1 * id_count;
    }
    similarity_score
}

fn main() {

    // reading input.txt and split by whitespace
    let file_path = "input.txt";
    let (left_list, right_list) = file_to_list(file_path);

//    // sorting
//    left_list.sort();
//    right_list.sort();

    // get the similarity score
    let similarity_score = get_similarity_score(left_list, right_list);

    println!("SIMILARITY SCORE: {:?}", similarity_score);
}
