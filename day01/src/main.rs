use std::collections::{BinaryHeap, HashMap};
use std::io::{self, BufRead};

fn main() {
    // let _ = solve_part1();
    let _ = solve_part2();
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

fn solve_part2() -> io::Result<()> {
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

    let mut right_frequency = HashMap::new();

    for _ in 0..right_heap.len() {
        let right = right_heap.pop().unwrap();

        if right_frequency.contains_key(&right) {
            let count = right_frequency.get(&right).unwrap();
            right_frequency.insert(right, count + 1);
        } else {
            right_frequency.insert(right, 1);
        }
    }

    let mut similarity = 0;
    for _ in 0..left_heap.len() {
        let left = left_heap.pop().unwrap();

        if let Some(count) = right_frequency.get(&left) {
            similarity += left * count;
        }
    }

    println!("{}", &similarity);

    Ok(())
}
