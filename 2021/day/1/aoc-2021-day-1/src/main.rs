use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() {
    let _text = read_input().expect("could not read input file!");
}


fn read_input() -> Result<Vec<String>, io::Error> {
    let file = File::open("../input")?;
    let buffer = BufReader::new(file);

    let mut lines = Vec::new();

    for line in buffer.lines() {
        if let Some(line) = &line.ok() {
            lines.push(line.clone());

            println!("{:?}", &line);
        }
    }
    Ok(lines)
}
