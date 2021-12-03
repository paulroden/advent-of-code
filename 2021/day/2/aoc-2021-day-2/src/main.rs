use std::num::ParseIntError;
use std::path::Path;
use std::str::FromStr;

fn main() {

    let input_file_path = Path::new("../input");

    let movements = input::read_lines(input_file_path, Movement::from_str)
        .expect("Error parsing input to list of movements");

    let initial_position = Position::new(0, 0, 0);
    let end_position = movements.iter().fold(initial_position, |p, displacement| p.displace(displacement));

    println!("{:?}", end_position);
    println!("{:?}", end_position.square_norm() );
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

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Position {
    x: i32,
    y: i32,
    aim: i32,
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}; {})", self.x, self.y, self.aim)
    }
}

impl Position {
    pub fn new(x: i32, y: i32, aim: i32) -> Self {
        Self { x, y, aim }
    }
    pub fn displace(&self, movement: &Movement) -> Self {
        match movement.direction {
            Direction::Forward => Position::new(
                self.x + movement.distance,
                self.y + (movement.distance * self.aim),
                self.aim,
            ),
            Direction::Backward => self.clone(),
            Direction::Up => Position::new(
                self.x,
                self.y,
                self.aim - movement.distance,
            ),
            Direction::Down => Position::new(
                self.x,
                self.y,
                self.aim + movement.distance,
            ),
        }
    }
    pub fn square_norm(&self) -> i32 {
        self.x * self.y
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
pub struct Movement {
    direction: Direction,
    distance: i32,
}

impl Movement {
    pub fn new(direction: Direction, distance: i32) -> Self {
        Self {
            direction,
            distance,
        }
    }
}

impl FromStr for Movement {
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
    let parsed = Movement::from_str(&line).unwrap();
    assert_eq!(
        parsed,
        Movement {
            direction: Direction::Forward,
            distance: 7
        }
    );
}

#[test]
fn can_parse_multiple_lines() {
    use crate::Direction::*;
    let input_text = "down 5\nforward 7\nforward 3\n";
    let expected = vec![
        Movement::new(Down, 5),
        Movement::new(Forward, 7),
        Movement::new(Forward, 3),
    ];

    let movements = input_text
        .lines()
        .map(|line| Movement::from_str(line).unwrap())
        .collect::<Vec<_>>();

    assert_eq!(movements, expected);
}


#[test]
fn movement_displces_position() {
    use crate::Direction::*;
    let initial_position = Position::new(0, 0, 0);
    let movements = vec![
        Movement::new(Forward, 5),
        Movement::new(Down, 5),
        Movement::new(Forward, 8),
        Movement::new(Up, 3),
        Movement::new(Down, 8),
        Movement::new(Forward, 2),
    ];

    let end_position = movements.iter().fold(initial_position, |p, displacement| p.displace(displacement));

    assert_eq!(end_position, Position::new(15, 60, 10));
}