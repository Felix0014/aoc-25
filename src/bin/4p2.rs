use std::fs;

fn main() {
    let mut shelf = fs::read_to_string("input/4.txt")
        .expect("Failed to read file")
        .lines()
        .map(|line| line.chars().map(|c| c == '@').collect::<Vec<bool>>())
        .collect::<Vec<Vec<bool>>>();
    let mut new_shelf: Vec<Vec<bool>>;
    let mut total = 0;
    let mut removed = 0;
    loop {
        new_shelf = shelf.clone();
        for x in 0..shelf.len() {
            for y in 0..shelf[0].len() {
                if shelf[x][y] && shelf.count_adjacent_rolls(x, y) < 4 {
                    removed += 1;
                    new_shelf[x][y] = false;
                }
            }
        }
        if removed == 0 {
            break;
        }
        total += removed;
        removed = 0;
        shelf = new_shelf;
    }
    println!("{}", total);
}

pub trait Shelf {
    fn check_fields(&self, x: usize, y: usize) -> bool;
    fn count_adjacent_rolls(&self, x: usize, y: usize) -> u32;
}

impl Shelf for Vec<Vec<bool>> {
    fn check_fields(&self, x: usize, y: usize) -> bool {
        self.get(x).and_then(|l| l.get(y)).copied().unwrap_or(false)
    }
    fn count_adjacent_rolls(&self, x: usize, y: usize) -> u32 {
        let mut sum = 0 as u32;
        let fields_x = 1 as i32;
        let fields_y = 1 as i32;
        for offset_x in -fields_x..fields_x + 1 {
            for offset_y in -fields_y..fields_y + 1 {
                if (offset_x == 0 && offset_y == 0)
                    || x as i32 + offset_x < 0
                    || y as i32 + offset_y < 0
                {
                    continue;
                }
                sum += self.check_fields(
                    (x as i32 + offset_x) as usize,
                    (y as i32 + offset_y) as usize,
                ) as u32;
            }
        }
        sum
    }
}
