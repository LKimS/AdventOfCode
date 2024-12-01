use std::fs;

fn main() -> std::io::Result<()> {
    // Specify the file path
    let file_path = "input.txt";
    
    // Read the entire file as a string
    let contents = fs::read_to_string(file_path)?;

    // split the contents of the file into a vector of lines
    let lines: Vec<&str> = contents.lines().collect();

    let col1: Vec<i32> = lines.iter().map(|line| {
        let nums: Vec<_> = line.split_whitespace().collect();
        let num1 = nums[0].parse::<i32>().unwrap();
        num1
    }).collect();

    let col2: Vec<i32> = lines.iter().map(|line| {
        let nums: Vec<_> = line.split_whitespace().collect();
        let num2 = nums[1].parse::<i32>().unwrap();
        num2
    }).collect();

    // sort the columns
    let mut col1 = col1;
    col1.sort();
    let mut col2 = col2;
    col2.sort();

    // find the difference between the two columns
    let s: i32 = col1.iter().zip(col2.iter()).map(|(a, b)| (a - b).abs()).sum();

    println!("Part 1: {}", s);

    
    // Part 2
    let s2 = col1.iter().map(|num1| {
        let count = col2.iter().filter(|&num2| num2 == num1).count() as i32;
        count * num1
    }).sum::<i32>();

    println!("Part 2: {}", s2);

    Ok(())
}
