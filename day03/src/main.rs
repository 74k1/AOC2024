use std::fs;
use std::env;

fn part1(input: &str) -> i32 {
    let mut sum = 0;
    for part in input.split("mul(") {
        // Skip the first part since it's before any "mul("
        if !part.contains(')') {
            continue;
        }

        if let Some(nums) = part.split(')').next() {
            // Split the numbers by comma
            let numbers: Vec<&str> = nums.split(",").collect();
            if numbers.len() == 2 {
                // Try to parse both numbers
                if let (Ok(n1), Ok(n2)) = (
                    numbers[0].trim().parse::<i32>(),
                    numbers[1].trim().parse::<i32>()
                ) {
                    sum += n1 * n2;
                }
            }
        }
    }
    sum
}

fn part2(input: &str) -> i32 {
    let mut sum = 0;
    let mut enabled = true;

    for part in input.split(|c| c == 'm' || c == 'd') {
        match part {
            p if p.starts_with("o()") => enabled = true,
            p if p.starts_with("on't()") => enabled = false,
            p if p.starts_with("ul(") && enabled => {
                if let Some(nums) = part[3..].split(')').next() {
                    let numbers: Vec<&str> = nums.split(',').collect();
                    if numbers.len() == 2 {
                        if let (Ok(n1), Ok(n2)) = (
                            numbers[0].trim().parse::<i32>(),
                            numbers[1].trim().parse::<i32>()
                        ) {
                            sum += n1 * n2;
                        }
                    }
                }
            },
            _ => () // ignore everything else
        }
    }
    sum
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Usage: cargo run <file_path> <part>");
        return;
    }

    let file_path = &args[1];
    let part = &args[2].parse::<i32>().expect("Part must be a number. (1 or 2)");

    let input = fs::read_to_string(file_path)
        .expect("Could not read file");

    let result = match part {
        1 => part1(&input),
        2 => part2(&input),
        _ => {
            println!("Invalid part number. Use 1 or 2");
            return;
        }
    };

    println!("Part {}: {}", part, result);
}
