use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

fn main() {
    let file = File::open("input/3.txt").unwrap();
    let reader = BufReader::new(file);
    let mut joltage_sum = 0;
    for line in reader.lines() {
        let max_sum = get_max_sum(line.unwrap().as_str());
        println!("{}", max_sum);
        joltage_sum += max_sum;
    }
    println!("sum: {}", joltage_sum);
}

fn get_max_sum(bank: &str) -> i32 {
    let max_level = bank[..bank.len() - 1].bytes().max().unwrap();
    let max_idx = bank.find(max_level as char).unwrap();
    let max_level_2 = bank[max_idx + 1..].bytes().max().unwrap();

    (max_level as i32 - '0' as i32) * 10 + (max_level_2 as i32 - '0' as i32)
}
