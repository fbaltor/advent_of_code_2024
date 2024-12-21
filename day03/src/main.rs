use regex::Regex;
use std::fs;

fn main() {
    let _ = part_2();
}

fn part_1() -> std::io::Result<()> {
    let file_path = "input.txt";
    let content = fs::read_to_string(file_path)?;

    let pattern = r"mul\(\d{1,3},\d{1,3}\)";
    let re = Regex::new(pattern).unwrap();

    let mut sum = 0;
    for caps in re.captures_iter(&content) {
        if let (Ok(first_number), Ok(second_number)) =
            (caps[1].parse::<i32>(), caps[2].parse::<i32>())
        {
            sum += first_number * second_number;
        } else {
            println!("Failed to parse one of the numbers.");
        }
    }

    println!("{}", sum);

    Ok(())
}

fn part_2() -> std::io::Result<()> {
    let file_path = "input.txt";
    let content = fs::read_to_string(file_path)?;

    let pattern_mul = r"mul\((\d{1,3}),(\d{1,3})\)";
    let pattern_dont = r"(don\'t\(\))";
    let pattern_do = r"(do\(\))";
    let pattern = format!(r"{}|{}|{}", pattern_mul, pattern_dont, pattern_do);
    println!("Regex pattern: {}", pattern); // Debug log for regex

    let re = Regex::new(&pattern).unwrap();

    let mut sum = 0;
    let mut is_active = true;
    for mat in re.captures_iter(&content) {
        if let (Some(mul_1), Some(mul_2)) = (mat.get(1), mat.get(2)) {
            if let (Ok(num_1), Ok(num_2)) =
                (mul_1.as_str().parse::<i32>(), mul_2.as_str().parse::<i32>())
            {
                if is_active {
                    sum += num_1 * num_2;
                }
            }
        } else if let Some(_) = mat.get(3) {
            is_active = false;
        } else if let Some(_) = mat.get(4) {
            is_active = true;
        }
    }

    println!("{}", sum);

    Ok(())
}
