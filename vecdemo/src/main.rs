
use std::collections::HashMap;

fn calculate_median( numbers: &Vec<i32>) -> f64 {
    let mut sort_numbers = numbers.clone();
    
    sort_numbers.sort();

    let length = sort_numbers.len();

    if length % 2 == 0 {
        let mid_right = length / 2;
        let mid_left = mid_right - 1;
        return (sort_numbers[mid_left] + sort_numbers[mid_right]) as f64/2.0;
    } else {
        return sort_numbers[length / 2] as f64;
    }
}

fn calculate_mode (numbers: &Vec<i32>) -> i32 {
    let mut count_map = HashMap::new();

    for &num in numbers {
        *count_map.entry(num).or_insert(0) += 1;
    }

    let mut mode = 0;
    let mut max_count = 0;

    for (&num, &count) in &count_map {
        if count > max_count {
            mode = num;
            max_count = count;
        }
    }
    mode
}

fn main() {
    let numbers = vec![1, 3, 2, 2, 4, 5, 3, 2, 2, 6, 7, 8, 9];

    let median = calculate_median(&numbers);
    println!("Median: {}", median);

    println!("Hello, world!");

    let mode = calculate_mode(&numbers);

    println!("Mode: {}", mode);
}
