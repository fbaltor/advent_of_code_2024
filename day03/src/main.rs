use regex::Regex;
use std::fs;

fn main() -> std::io::Result<()> {
    let file_path = "input.txt";
    let content = fs::read_to_string(file_path)?;

    let pattern = r"mul\((\d{1,3}),(\d{1,3})\)";
    let re = Regex::new(pattern).unwrap();

    let mut sum = 0;
    for caps in re.captures_iter(&content) {
        if let (Ok(first_number), Ok(second_number)) =
            (caps[1].parse::<i32>(), caps[2].parse::<i32>())
        {
            println!("{} {}", first_number, second_number);
            sum += first_number * second_number;
        } else {
            println!("Failed to parse one of the numbers.");
        }
    }

    println!("{}", sum);

    Ok(())
}
