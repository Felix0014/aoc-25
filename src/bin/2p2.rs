use std::{error::Error, fs};

type NumType = i128;

fn main() -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string("input/2.txt")?;
    let ranges = content.split(",");
    let mut invalid_id_sum: NumType = 0;
    for range in ranges {
        let (start, end) = match parse_range(range) {
            Some(pair) => pair,
            None => {
                println!("invalid range: {}", range);
                continue;
            }
        };
        invalid_id_sum += (start..=end).filter(is_invalid_id).sum::<NumType>();
    }
    println!("{}", invalid_id_sum);
    Ok(())
}

fn parse_range(range: &str) -> Option<(NumType, NumType)> {
    Some((
        range
            .split("-")
            .nth(0)
            .and_then(|s| s.parse::<NumType>().ok())?,
        range
            .split("-")
            .nth(1)
            .and_then(|s| s.parse::<NumType>().ok())?,
    ))
}

fn is_invalid_id(id: &NumType) -> bool {
    let id_str = id.to_string();
    for i in 1..=(id_str.len() / 2) {
        if id_str.len() % i != 0 {
            continue;
        }
        let mut idx = 0;
        loop {
            idx += i;
            if idx == id_str.len() {
                return true;
            }
            if &id_str[idx..idx + i] != &id_str[idx - i..idx] {
                break;
            }
        }
    }
    false
}
