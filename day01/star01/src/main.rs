use std::fs;
use std::env;

fn main() {
    // 1. read whole file as string
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Please provide a file path");
        return;
    }

    let file_path = &args[1];

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

    // 4. sort both lists
    list1.sort();
    list2.sort();


    // 5. ZIP???
    let total: i32 = list1.iter()
        .zip(list2.iter())
        .map(|(a, b)| (a - b).abs())
        .sum();

    //// 5. calculate absolute from both lists at the same time
    //let mut absolutes = Vec::new();
    //for i in 0..list1.len() {
    //    let diff = (list1[i] - list2[i]).abs();
    //    absolutes.push(diff);
    //}
    //
    //// 6. add up those absolutes
    //let total: i32 = absolutes.iter().sum();

    // 7. result
    println!("{total}");
}
