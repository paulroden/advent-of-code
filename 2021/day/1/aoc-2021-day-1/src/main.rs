use std::cmp::Ordering;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::path::Path;

fn main() {
    let input_file_path = Path::new("../input");
    let input_values = read_input_lines(input_file_path).expect("could not read input file!");

    let window_totals = input_values
        .windows(3)
        .map(|window| window.iter().sum())
        .collect::<Vec<i32>>();

    let increase_count = window_totals
        .windows(2)
        .map(|pair| {
            let next = pair[1];
            let current = pair[0];
            match next.cmp(&current) {
                Ordering::Greater => 1,
                _ => 0,
            }
        })
        .sum::<u32>();

    println!("{}", increase_count);
}

fn read_input_lines(path: &Path) -> Result<Vec<i32>, io::Error> {
    let file = File::open(path)?;
    let buffer = BufReader::new(file);

    let mut lines = Vec::new();

    for line in buffer.lines() {
        if let Some(line) = &line.ok() {
            if let Ok(number) = line.parse::<i32>() {
                lines.push(number)
            }
        }
    }
    Ok(lines)
}
