use std::io::{self, BufRead};

fn main() {
    _ = solve_part1();
}

fn solve_part1() -> io::Result<()> {
    let stdin = io::stdin();

    let mut safe_reports = 0;
    for line in stdin.lock().lines() {
        let line = line?;

        let report: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse::<i32>().expect("Failed to parse number"))
            .collect();

        if report_is_safe(report) {
            safe_reports += 1;
        }
    }

    println!("{}", safe_reports);

    Ok(())
}

fn report_is_safe(v: Vec<i32>) -> bool {
    let mut previous = v[0];
    let mut start = 1;
    let is_increasing = v[start] - previous > 0;

    while start < v.len() {
        let keeps_increasing = v[start] - previous > 0; 

        if is_increasing != keeps_increasing {
            return false
        }

        let transition = (v[start] - previous).abs();
        let transition_is_safe = 0 < transition && transition < 4;

        if !transition_is_safe {
            return false
        }

        previous = v[start];
        start += 1;
    }

    return true;
}
