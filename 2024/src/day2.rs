use crate::utils::read_data;

static PATH: &str = "./input/day2";

// O(n^2)
pub fn part1() -> (i32, i32) {
    let mut reports: Vec<Vec<i32>> = Vec::new();
    let mut safe_reports = 0;
    let mut unsafe_reports = 0;

    get_data(PATH, &mut reports);
    // Iterating over reports
    for report in reports {
        let mut increasing = true;
        let mut is_safe = true;
        // Knowing if the report is increasing or decreasing
        if report[0] > report[1] {
            increasing = false;
        }

        for level in 0..report.len() - 1 {
            // Checking if still in the order
            let safety = if increasing {
                report[level] < report[level + 1]
            } else {
                report[level] > report[level + 1]
            };

            if safety {
                // in case is not safety just go to ther report
                if !check_safety(report[level], report[level + 1]) {
                    is_safe = false;
                    break;
                } else {
                    continue;
                }
            } else {
                is_safe = false;
                break;
            }
        }
        // Add report status
        if is_safe {
            safe_reports += 1;
        } else {
            unsafe_reports += 1;
        }
    }

    (safe_reports, unsafe_reports)
}

pub fn part2() -> (i32, i32) {
    let mut reports: Vec<Vec<i32>> = Vec::new();
    let mut safe_reports = 0;
    let mut unsafe_reports = 0;

    get_data(PATH, &mut reports);
    // Iterating over reports
    for mut report in reports {
        let mut increasing: bool = false;

        let mut is_safe = true;
        let mut single_bad_level = false;
        let mut last_level: bool;

        // Knowing if the report is increasing or decreasing
        for level in 0..report.len() - 1 {
            last_level = increasing;
            increasing = report[level] < report[level + 1];
            println!("{}", increasing);

            let safety = if increasing {
                report[level] < report[level + 1]
            } else {
                report[level] > report[level + 1]
            };

            if !safety {
                if single_bad_level {
                    is_safe = false;
                    break;
                } else {
                    single_bad_level = true;
                    // Moving actual element to next
                    report[level + 1] = report[level];
                }
            } else {
                if !increasing && last_level {
                    println!("llegue");
                    report[level] = report[level - 1];
                }
                // in case is not safety just go to ther report
                if !check_safety(report[level], report[level + 1]) {
                    if single_bad_level {
                        is_safe = false;
                        break;
                    } else {
                        single_bad_level = true;
                        if level == 0 {
                            continue;
                        }
                    }
                } else {
                    continue;
                }
            }
        }
        // Add report status
        if is_safe {
            println!("[{:?}] safe", report);
            safe_reports += 1;
        } else {
            println!("[{:?}] unsafe", report);
            unsafe_reports += 1;
        }
    }

    (safe_reports, unsafe_reports)
}

// Just chech if the difference is okay
fn check_safety(day: i32, next_day: i32) -> bool {
    let differ = (day - next_day).abs();
    // equals to this: 1 <= differ && differ <= 3
    if (1..=3).contains(&differ) {
        return true;
    }
    false
}

fn simulate_change(report: &Vec<i32>, level: usize, increasing: bool) -> bool {
    let mut temp_report = report.clone();
    temp_report[level + 1] = temp_report[level]; // Simula el cambio
                                                 // Verifica si el reporte simulado es seguro
    for i in 0..temp_report.len() - 1 {
        let is_safe = if increasing {
            temp_report[i] < temp_report[i + 1]
        } else {
            temp_report[i] > temp_report[i + 1]
        };
        if !is_safe {
            return false;
        }
    }
    true
}

fn get_data(str_path: &str, reports: &mut Vec<Vec<i32>>) {
    let lines = read_data(str_path);
    for line in lines {
        let mut temp: Vec<i32> = Vec::new();
        for part in line.split_whitespace() {
            if !part.is_empty() {
                temp.push(part.parse::<i32>().unwrap());
            }
        }
        reports.push(temp);
    }
}
