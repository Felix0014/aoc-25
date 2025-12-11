use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

fn main() -> io::Result<()> {
    let file = File::open("input/1.txt")?;
    let reader = BufReader::new(file);
    let mut current_code = 50;
    let mut code = 0;
    for line in reader.lines() {
        let line = line?;
        let direction = line[0..1].parse::<char>().unwrap();
        let mut val = match line[1..].parse::<i32>() {
            Ok(v) => v,
            Err(_) => continue,
        };
        current_code += if direction == 'R' { val } else { -val };
        current_code += if current_code < 0 { 100 } else { 0 };
        current_code %= 100;
        if (current_code == 0) {
            code += 1;
        }
    }
    println!("{}", code);
    Ok(())
}
