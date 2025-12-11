use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

type NumType = i128;

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

fn get_max_sum(bank: &str) -> NumType {
    let mut limit_right = bank.len() - 11;
    let mut limit_left = 0;
    let mut sum = 0;

    while limit_right != bank.len() + 1 {
        let max = bank[limit_left..limit_right].bytes().max().unwrap();
        println!(
            "search_window:\n{}\n{}{}\nmax: {}\n",
            bank,
            " ".repeat(limit_left),
            bank[limit_left..limit_right].to_string(),
            max as char
        );
        limit_left = bank[limit_left..limit_right].find(max as char).unwrap() + limit_left + 1;
        sum += (max as NumType - '0' as NumType) * 10_i128.pow((bank.len() - limit_right) as u32);
        limit_right += 1;
    }
    return sum;
}
