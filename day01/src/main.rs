use std::collections::BinaryHeap;
use std::io::{self, BufRead};

fn main() {
    let _ = solve_part1();
}

fn solve_part1() -> io::Result<()> {
    let stdin = io::stdin();

    let mut left_heap: BinaryHeap<i32> = BinaryHeap::new();
    let mut right_heap: BinaryHeap<i32> = BinaryHeap::new();

    for line in stdin.lock().lines() {
        let line = line?;

        let numbers: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse::<i32>().expect("Failed to parse number"))
            .collect();

        if numbers.len() == 2 {
            left_heap.push(numbers[0]);
            right_heap.push(numbers[1]);
        } else {
            eprintln!("Unexpected line format: {}", line);
        }
    }

    let mut total = 0;

    for _ in 0..left_heap.len() {
        total += (left_heap.pop().unwrap() - right_heap.pop().unwrap()).abs();
    }

    println!("{}", &total);

    Ok(())
}
