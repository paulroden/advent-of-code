use std::num::ParseIntError;
use std::str::FromStr;

fn main() {
    let s = "forward 7".to_string();

    let d = DirectionalMovement::from_str(&s);

    println!("{:?}", d);
}

#[derive(Debug)]
enum ParseError {
    ParseIntError(ParseIntError),
    ParseDirectionError,
}

impl From<ParseIntError> for ParseError {
    fn from(err: ParseIntError) -> Self {
        ParseError::ParseIntError(err)
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Direction {
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
struct DirectionalMovement {
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
    use std::fs::File;
    use std::io::{self, prelude::*, BufReader};
    use std::path::Path;

    fn read_lines<T, P>(path: &Path, line_parser: P) -> Result<Vec<T>, io::Error>
    where
        P: FnOnce(&str) -> T,
        T: std::str::FromStr,
    {
        let file = File::open(path)?;
        let buffer = BufReader::new(file);

        let mut lines = Vec::new();

        for line in buffer.lines() {
            if let Some(line) = &line.ok() {
                if let Ok(number) = line.parse::<T>() {
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
