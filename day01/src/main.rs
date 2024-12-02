use std::fs;
use std::env;

fn part1(list1: &Vec<i32>, list2: &Vec<i32>) -> i32 {
    let mut sorted1 = list1.clone();
    let mut sorted2 = list2.clone();

    sorted1.sort();
    sorted2.sort();

    sorted1.iter()
        .zip(sorted2.iter())
        .map(|(a, b)| (a - b).abs())
        .sum()
}

fn part2(list1: &Vec<i32>, list2: &Vec<i32>) -> i32 {
    list1.iter()
        .map(|&num| {
            let appearances = list2.iter().filter(|&&x| x == num).count();
            num * (appearances as i32)
        })
        .sum()
}

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

    // 2. create two empty lists
    let mut list1 = Vec::new();
    let mut list2 = Vec::new();

    // 3. go through each line of the file
    for line in input.lines() {
        // split the line into two parts where there's spaces
        let mut pair = line.split_whitespace();

        // get the numbers and add them to our lists
        let num1 = pair.next().unwrap().parse::<i32>().unwrap();
        let num2 = pair.next().unwrap().parse::<i32>().unwrap();

        list1.push(num1);
        list2.push(num2);
    }

    // 4. Calculate based on part

    let result = match part {
        1 => part1(&list1, &list2),
        2 => part2(&list1, &list2),
        _ => {
            println!("Invalid part number. Use 1 or 2");
            return;
        }
    };

    println!("Part {}: {}", part, result);
}
