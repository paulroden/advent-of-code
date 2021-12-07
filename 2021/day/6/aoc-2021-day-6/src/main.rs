use std::fmt;
use std::num::ParseIntError;
use std::str::FromStr;

fn main() {
    println!("Hello, world!");
}

#[derive(Debug, Clone)]
struct LanternFish {
    time: u32,
}

impl fmt::Display for LanternFish {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.time)
    }
}

impl FromStr for LanternFish {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let time = s.parse::<u32>()?;
        Ok(Self { time })
    }
}

impl LanternFish {
    fn new(time: u32) -> Self {
        Self { time }
    }

    fn next_day(&self) -> (Option<Self>, Option<Self>) {
        match self.time {
            0 => (Some(Self { time: 6 }), Some(Self { time: 8 })),
            _ => (
                Some(Self {
                    time: self.time - 1,
                }),
                None,
            ),
        }
    }
}

#[derive(Debug)]
struct School {
    fish: Vec<LanternFish>,
}

impl fmt::Display for School {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let fish_times = self
            .fish
            .iter()
            .map(|a| a.time)
            .collect::<Vec<_>>();
        write!(f, "{:?}", fish_times)
    }
}

impl FromStr for School {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let fish: Result<Vec<_>, Self::Err> =
            s.split(',').map(|ch| LanternFish::from_str(ch)).collect();
        match fish {
            Ok(fish) => Ok(Self { fish }),
            Err(e) => Err(e),
        }
    }
}

impl School {
    fn from_fish(fish_times: &[u32]) -> Self {
        let fish = fish_times
            .iter()
            .map(|i| LanternFish::new(*i))
            .collect::<Vec<_>>();
        Self { fish }
    }

    fn next_day(&self) -> Self {
        let next_school = self
            .fish
            .iter()
            .flat_map(|a| match a.next_day() {
                (Some(fish), Some(baby_fish)) => vec![fish, baby_fish],
                (Some(fish), None) => vec![fish],
                (None, _) => vec![],
            })
            .collect::<Vec<_>>();
        Self { fish: next_school }
    }

    fn count(&self) -> usize {
        self.fish.len()
    }
}

mod tests {
    use crate::*;
    #[test]
    fn single_fish() {
        let school = School::from_str("4").unwrap();

        let mut next_school = school;

        for _ in 0..6 {
            println!("{}", next_school);
            next_school = next_school.next_day();
        }
    }

    #[test]
    fn example() {
        let input = "3,4,3,1,2".to_string();
        let initial_state = School::from_str(&input).unwrap();

        let final_state = (0..18).fold(initial_state, |school, _| school.next_day());
        // let mut next_school = initial_state;

        println!("{}", final_state);

        for 0..18
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
