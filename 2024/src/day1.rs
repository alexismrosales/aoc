use crate::utils;
use std::{collections::HashMap, vec::Vec};

static PATH: &str = "./input/day1";
// Approx complexity O(nlogn) with n as size of any of both vectors
pub fn part1() -> i32 {
    let mut left_side: Vec<i32> = Vec::new();
    let mut right_side: Vec<i32> = Vec::new();
    get_data(PATH, &mut left_side, &mut right_side);

    // Sort data to pair every numbers correctly
    left_side.sort();
    right_side.sort();
    println!("l {:?}, r {:?}", left_side, right_side);
    let mut total_distance: i32 = 0;
    for i in 0..left_side.len() {
        // Obtain distance
        total_distance += (left_side[i] - right_side[i]).abs();
    }

    total_distance
}

// Approx complexity O(n+m) but n = m, so O(2n) -> O(n)
pub fn part2() -> i32 {
    let mut left_side: Vec<i32> = Vec::new();
    let mut right_side: Vec<i32> = vec![]; // Other way using macro to declare a emptty vector

    get_data(PATH, &mut left_side, &mut right_side);

    let mut frequencies: HashMap<i32, i32> = HashMap::new();
    // Saving coincidences
    for &num in &right_side {
        *frequencies.entry(num).or_insert(0) += 1;
    }

    let mut similarity: i32 = 0;
    // Finding an computing coincidences if exist in left side
    for num in left_side {
        if let Some(&count) = frequencies.get(&num) {
            similarity += num * count;
        }
    }

    similarity
}

// Obtaing data from input file
fn get_data(str_path: &str, left_side: &mut Vec<i32>, right_side: &mut Vec<i32>) {
    let lines = utils::read_data(str_path);
    // Iterating over lines
    for line in lines {
        // Saving numbers in vector
        for (i, part) in line.split_whitespace().enumerate() {
            if !part.is_empty() {
                if i == 0 {
                    left_side.push(part.parse().expect("Failed to parse string."));
                } else {
                    right_side.push(part.parse().expect("Failed to parse string."));
                }
            }
        }
    }
}
