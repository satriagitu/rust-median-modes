use std::collections::HashMap;

fn calculate_median_and_mode(numbers: &mut Vec<i32>) -> (f32, Vec<i32>) {
    // Sort the numbers for median calculation
    numbers.sort();

    // Calculate the median
    let median = if numbers.len() % 2 == 0 {
        let mid1 = numbers[numbers.len() / 2 - 1];
        let mid2 = numbers[numbers.len() / 2];
        (mid1 + mid2) as f32 / 2.0
    } else {
        numbers[numbers.len() / 2] as f32
    };

    // Calculate the mode
    let mut occurrences = HashMap::new();
    for num in numbers.iter() {
        *occurrences.entry(*num).or_insert(0) += 1;
    }

    // Find the highest frequency
    let max_frequency = occurrences.values().cloned().max().unwrap_or(0);

    // Collect all numbers with the highest frequency
    let modes: Vec<i32> = occurrences
        .iter()
        .filter(|&(_, &count)| count == max_frequency)
        // .filter(count,)
        .map(|(&num, _)| num)
        .collect();

    // let iter = occurrences.iter();

    // for (key, val) in occurrences.iter() {
    //     println!("key: {key} val: {val}");
    // }
    // // let filtered = iter.filter(|&(_, &count)| count == max_frequency);
    // let filtered = iter.filter(|&(_, &count)| count == max_frequency);
    // let mapped = filtered.map(|(&num, _)| num);
    // let modes: Vec<i32> = mapped.collect();

    (median, modes)
}

fn calculate_median(numbers: &mut Vec<i32>) -> f32 {
    numbers.sort();

    let median = if numbers.len() % 2 == 0 {
        let mid1 = numbers[(numbers.len() / 2) - 1];
        let mid2 = numbers[numbers.len() / 2];
        (mid1 + mid2) as f32 / 2.0
    } else {
        numbers[numbers.len() / 2] as f32
    };

    median
}

fn calculate_mode(numbers: &mut Vec<i32>) -> Vec<i32> {
    // Calculate the mode (most frequent numbers)
    let mut occurences = HashMap::new();
    for &num in numbers.iter() {
        *occurences.entry(num).or_insert(0) += 1;
    }

    // Find the maximum frequency (highest count)
    let mut max_count = 0;
    for &count in occurences.values() {
        if count > max_count {
            max_count = count;
        }
    }

    // Collect all values with the maximum frequency
    let mut mode = Vec::new();
    for (&num, &count) in occurences.iter() {
        if count == max_count {
            mode.push(num);
        }
    }

    mode
}

fn main() {
    let mut numbers = vec![1, 2, 2, 3, 3, 3, 4, 4, 4, 5, 6];
    let (median, modes) = calculate_median_and_mode(&mut numbers);

    println!("Median: {}", median);
    println!("Mode: {:?}", modes);

    let median2 = calculate_median(&mut numbers);
    let mode2 = calculate_mode(&mut numbers);

    println!("Median: {}", median2);
    println!("Mode: {:?}", mode2);
}
