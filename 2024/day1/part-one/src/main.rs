// adventofcode DAY1-1
//
// Read two lists from a file containing numbers.
// Pair up the smallest number in the left list with the smallest number in the right list,
// then the second-smallest left number with the second-smallest right number, and so on.
//
// Compute the total distance by adding up all the distances from the sorted lists.
//
// SOLUTION:
// The total distance is 2192892



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

fn get_lists_distance(list1: Vec<i32>, list2: Vec<i32>) -> i32 {

    let mut total_distance = 0;

    if !list1.len() == list2.len() {
        panic!("Can't compare list distances of two lists with different lengths.");
    } else {
        for i in 0.. list1.len() {
            let distance = (list1[i] - list2[i]).abs();
            total_distance += distance;
        }
    }
    total_distance
}

fn main() {

    // reading input.txt and split by whitespace
    let file_path = "input.txt";
    let (mut left_list, mut right_list) = file_to_list(file_path);

    // sorting
    left_list.sort();
    right_list.sort();

    // get the total distance
    let total_distance = get_lists_distance(left_list, right_list);

    println!("TOTAL DISTANCE: {:?}", total_distance);
}
