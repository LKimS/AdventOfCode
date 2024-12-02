use std::fs;


fn main() {
   let contents = fs::read_to_string("input.txt").expect("Failed to read file");

    let lines: Vec<&str> = contents.lines().collect();

    let filter_nums: Vec<String> = lines.iter().map(|line| {
        // only filter out the numbers in from the line
        // asdf12asdf3 -> 13
        let digits = line.chars().filter(|c| c.is_numeric()).collect::<String>();
        
        if digits.len() > 2 {
            // return first and last digit
            let first = digits.chars().next();
            let last = digits.chars().last();
            let ret = format!("{}{}", first.unwrap(), last.unwrap());
            return ret;

        }
        else if digits.len() == 1 {
            let first = digits.chars().next();
            return format!("{}{}", first.unwrap(), first.unwrap());
        }
        else {
            return digits;
        }
    
    }).collect();

    // filter num to i32
    let filter_nums:Vec<i32> = filter_nums.iter().map(|num| {
        num.parse::<i32>().unwrap()
    }).collect();

    println!("Part 1: {:?}", filter_nums.iter().sum::<i32>());

}
