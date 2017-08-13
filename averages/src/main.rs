use std::io;
use std::collections::HashMap;

fn main() {
    println!("Please enter a list of numbers separated by whitespace.");

    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    let mut num_list: Vec<i32> = Vec::new();

    for num_str in input.trim().split_whitespace() {
        let num = num_str.trim().parse()
                      .expect("Non-integer value entered");
        num_list.push(num);
    }
    num_list.sort();

    println!("Mean: {:?}", int_mean(&num_list));
    println!("Median: {:?}", int_median(&num_list));
    println!("Mode: {:?}", int_mode(&num_list));
}

fn int_mean(v: &Vec<i32>) -> f32 {
    let mut mean = 0;

    for n in v.iter() {
        mean = mean + n;
    }
    (mean as f32)/(v.len() as f32)
}

fn int_median(v: &Vec<i32>) -> f32 {
    match v.len() % 2 {
        1 => v[(v.len()-1)/2] as f32,
        0 => ((v[v.len()/2 - 1] + v[v.len()/2]) as f32)/2.0,
        _ => panic!("Bad error with the vector!"),
    }
}

fn int_mode(v: &Vec<i32>) -> Vec<i32> {
    let mut count_map = HashMap::new();

    for n in v.iter() {
        let count = count_map.entry(n).or_insert(0);
        *count += 1;
    }

    let largest_count = count_map.values().max();
    let mut result = Vec::new();
    for (key, val) in count_map.iter() {
        if Some(val) == largest_count {
            result.push(**key);
        }
    }
    result.sort();

    result
}
