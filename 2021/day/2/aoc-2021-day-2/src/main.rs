use std::num::ParseIntError;
use std::path::Path;
use std::str::FromStr;

fn main() {
    let s = "forward 7".to_string();

    let d = DirectionalMovement::from_str(&s);

    println!("{:?}", d);

    let input_file_path = Path::new("../input");

    let movements = input::read_lines(input_file_path, DirectionalMovement::from_str);

    println!("{:?}", movements);
}

#[derive(Debug)]
pub enum ParseError {
    ParseIntError(ParseIntError),
    ParseDirectionError,
}

impl From<ParseIntError> for ParseError {
    fn from(err: ParseIntError) -> Self {
        ParseError::ParseIntError(err)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Forward,
    Backward,
}

impl FromStr for Direction {
    type Err = crate::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "up" => Ok(Direction::Up),
            "down" => Ok(Direction::Down),
            "forward" => Ok(Direction::Forward),
            "backward" => Ok(Direction::Backward),
            _ => Err(ParseError::ParseDirectionError),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct DirectionalMovement {
    direction: Direction,
    distance: i32,
}

impl FromStr for DirectionalMovement {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pair = s.split(' ').collect::<Vec<_>>();
        let direction = Direction::from_str(pair[0])?;
        let distance = pair[1].parse::<i32>()?;
        Ok(Self {
            direction,
            distance,
        })
    }
}

mod input {
    use crate::ParseError;
    use std::fs::File;
    use std::io::{self, prelude::*, BufReader};
    use std::path::Path;

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
}

#[test]
fn can_parse_single_line() {
    let line = "forward 7".to_string();
    let parsed = DirectionalMovement::from_str(&line).unwrap();
    assert_eq!(
        parsed,
        DirectionalMovement {
            direction: Direction::Forward,
            distance: 7
        }
    );
}
