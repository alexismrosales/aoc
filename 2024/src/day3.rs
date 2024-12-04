use crate::utils::read_data;
use regex::Regex;

static PATH: &str = "./input/day3";

pub fn exec(part: usize) {
    let memory = get_data();
    if part == 1 {
        println!("result: {:?}", part1(memory));
    } else {
        println!("result: {:?}", part2(memory));
    }
}

fn part1(memory: Vec<String>) -> i64 {
    // regex for mul(...,...)
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut result: i64 = 0;
    for m in memory {
        for capture in re.captures_iter(&m) {
            let num1: i32 = capture.get(1).unwrap().as_str().parse().unwrap();
            let num2: i32 = capture.get(2).unwrap().as_str().parse().unwrap();
            let mult: i32 = num1 * num2;
            result += mult as i64;
        }
    }
    result
}

fn part2(memory: Vec<String>) -> i64 {
    let re = Regex::new(r"(mul\((\d{1,3}),(\d{1,3})\))|(don't)|(do)").unwrap();
    let mut result: i64 = 0;

    let mut is_valid = true;
    for m in memory {
        for capture in re.captures_iter(&m) {
            if capture.get(1).is_some() {
                if is_valid {
                    let num1: i32 = capture.get(2).unwrap().as_str().parse().unwrap();
                    let num2: i32 = capture.get(3).unwrap().as_str().parse().unwrap();
                    result += (num1 * num2) as i64;
                }
            } else if capture.get(4).is_some() {
                is_valid = false;
            } else if capture.get(5).is_some() {
                is_valid = true;
            }
        }
    }
    result
}
fn get_data() -> Vec<String> {
    read_data(PATH)
}

#[cfg(test)]
mod tests {
    use super::*;
    fn part1_test_1() {
        let memory1: Vec<String> = vec![String::from(
            "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))",
        )];
        let memory2: Vec<String> = vec![String::from("xmul(2,4)%mul(4*mul( 2, 4 )")];
        let result1 = part1(memory1);
        let result2 = part1(memory2);
        let expected1 = 161;
        let expected2 = 8;

        println!("result: {}, expected: {}", result1, expected1);
        println!("result: {}, expected: {}", result2, expected2);

        assert_eq!(result1, expected1);
        assert_eq!(result2, expected2);
    }
    #[test]
    fn part2_test_1() {
        let memory1: Vec<String> = vec![String::from(
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))",
        )];
        let memory2: Vec<String> = vec![String::from("xdon't()mul(2,4)%mul(4*mul( 2, 4 )")];
        let result1 = part2(memory1);
        let result2 = part2(memory2);
        let expected1 = 48;
        let expected2 = 0;

        println!("result: {}, expected: {}", result1, expected1);
        println!("result: {}, expected: {}", result2, expected2);

        assert_eq!(result1, expected1);
        assert_eq!(result2, expected2);
    }
}
