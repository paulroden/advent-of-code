use std::fs::File;
use std::path::Path;
use std::io::{self, prelude::*, BufReader};

fn main() {
    let input_file_path = Path::new("../input");
    let text = read_input_lines(input_file_path).expect("could not read input file!");

    println!("{:?}", text);
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
