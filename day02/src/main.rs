use std::fs;
use std::env;

fn part1(list: &Vec<Vec<i32>>) -> i32 {
    list.iter()
        .filter(|report| {
            // if less than 2 numbers, it's safe
            if report.len() < 2 {
                return true;
            }

            // get first difference to determine direction
            let first_diff = report[1] - report[0];

            // equal numbers are not allowed
            if first_diff == 0 {
                return false;
            }

            // are we increasing or decreasing?
            let is_inc = first_diff > 0;

            // check with each pair
            report.windows(2)
                .all(|pair| {
                    let diff = pair[1] - pair[0];

                    // check all rules at once:
                    // - must maintain direction
                    // - must differ by 1-3
                    !((is_inc && diff <= 0) || // wrong direction if increasing
                        (!is_inc && diff >= 0) || // wrong direction if decreasing
                        diff.abs() > 3) // too big difference
                })
        })
        .count() as i32
}

//fn part2(list: &Vec<Vec<i32>>) -> i32 {
//}

fn main() {
    // 1. read whole file as string
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Usage: cargo run <file_path> <part>");
        return;
    }

    let file_path = &args[1];
    let part = &args[2].parse::<i32>().expect("Part must be a number. (1 or 2)");

    let input = fs::read_to_string(file_path)
        .expect("Could not read file");

    // 2. make vectors
    let list: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect()
        })
        .collect();

    // 3. Calculate based on part

    let result = match part {
        1 => part1(&list),
        //2 => part2(&list),
        _ => {
            println!("Invalid part number. Use 1 or 2");
            return;
        }
    };

    println!("Part {}: {}", part, result);
}
