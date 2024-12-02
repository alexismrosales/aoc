use std::fs::read_to_string;

/* --------------- FILES FUNCTIONS ------------ */
pub fn read_data(str_path: &str) -> Vec<String> {
    read_to_string(str_path)
        .unwrap() // if there is file-reading error -> panic
        .lines() // split string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // Collecthing strings and gather them in a vector
}

/* --------------- ALGORITHMS ----------------- */
