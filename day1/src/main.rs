use std::fs;
use itertools::Itertools;


fn find_pair_equals_to(content: &str, target: i32, pair_size: usize) -> Option<Vec<i32>> {
    content 
        .lines()
        .map(|item| item.parse::<i32>().unwrap())
        .combinations(pair_size)
        .filter(|item| item.iter().sum::<i32>() == target)
        .next()
}


fn main() {
    const EXPENSES_FILENAME: &str = "resources/expenses";
    const TARGET: i32 = 2020;

    let content = fs::read_to_string(EXPENSES_FILENAME).unwrap();
    
    // Part 1
    match find_pair_equals_to(&content, TARGET, 2) {
        Some(x) => {
            println!("Found pair. Product is: {}", x.iter().product::<i32>());
        },
        None => println!("Didn't find any pair summing to {}", TARGET),
    };

    // Part 2
    match find_pair_equals_to(&content, TARGET, 3) {
        Some(x) => {
            println!("Found triple. Product is: {}", x.iter().product::<i32>());
        },
        None => println!("Didn't find any pair summing to {}", TARGET),
    };
}