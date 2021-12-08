use regex::{self, Regex};
use std::path::Path;
use std::str::FromStr;
use lazy_static::lazy_static;

fn main() {
    let input_file_path = Path::new("../input");
    let file_by_lines = input::read_lines(
        input_file_path,
        |line| Ok(LineVector::from_str(line).unwrap())
    )
    .expect("could not read input file");

    println!("{:?}", file_by_lines.len());

}


fn parse_captures_to_i32(captures: &regex::Captures, i: usize) -> i32 {
    captures.get(i).unwrap().as_str().parse::<i32>().unwrap()
}

#[derive(Debug, PartialEq)]
struct LineVector {
    start: (i32, i32),
    end: (i32, i32),
}

impl LineVector {
    fn new(start: (i32, i32), end: (i32, i32)) -> Self {
        Self { start, end }
    }
}

#[derive(Debug)]
struct LineParseError;

impl FromStr for LineVector {
    type Err = LineParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref PATTERN: Regex =
                Regex::new(r"^(\d+?),(\d+?) -> (\d+?),(\d+?)$").unwrap();
        }
        if let Some(captures) = PATTERN.captures(s) {
            let start = (
                parse_captures_to_i32(&captures, 1),
                parse_captures_to_i32(&captures, 2),
            );
            let end = (
                parse_captures_to_i32(&captures, 3),
                parse_captures_to_i32(&captures, 4),
            );
            Ok(Self { start, end })
        } else {
            Err(LineParseError)
        }
    }
}

mod tests {
    use std::str::FromStr;
    use crate::LineVector;

    #[test]
    fn parses_string() {
        let sample = "565,190 -> 756,381";
        assert_eq!(
            LineVector::from_str(sample).unwrap(),
            LineVector::new((565,190), (756,381))
        )
    }

    #[test]
    fn parses_lines() {
        let sample_lines = "565,190 -> 756,381\n402,695 -> 402,138\n271,844 -> 98,844\n";
        let parsed = sample_lines.lines().map(
            |line| LineVector::from_str(line).unwrap()
        )
        .collect::<Vec<_>>();
        
        assert_eq!(parsed.len(), 3);
        assert_eq!(
            parsed.first().unwrap().start,
            (565,190)
        );
        assert_eq!(
            parsed.last().unwrap().end,
            (98,844)
        );
    }
}


#[allow(dead_code)]
mod input {
    use std::fs::File;
    use std::io::{self, prelude::*, BufReader};
    use std::num::ParseIntError;
    use std::path::Path;

    #[derive(Debug)]
    pub enum ParseError {
        ParseIntError(ParseIntError),
    }

    impl From<ParseIntError> for ParseError {
        fn from(err: ParseIntError) -> Self {
            ParseError::ParseIntError(err)
        }
    }

    pub fn file_by_line(
        path: &Path,
    ) -> std::io::Result<impl Iterator<Item = Result<String, io::Error>>> {
        let file = File::open(path)?;
        let buffer = BufReader::new(file);
        Ok(buffer.lines())
    }

    pub fn read_lines<T, P>(path: &Path, line_parser: P) -> Result<Vec<T>, io::Error>
    where
        P: Fn(&str) -> Result<T, ParseError>,
        T: std::str::FromStr,
    {
        let file = File::open(path)?;
        let buffer = BufReader::new(file);
        let mut lines = Vec::new();

        for line in buffer.lines() {
            if let Some(line) = &line.ok() {
                if let Ok(number) = line_parser(line) {
                    lines.push(number)
                }
            }
        }
        Ok(lines)
    }

    pub fn read_as_string(path: &Path) -> Result<String, io::Error> {
        let mut file = File::open(path)?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;
        Ok(buffer)
    }
}
