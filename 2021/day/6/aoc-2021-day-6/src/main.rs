use std::fmt;
use std::num::ParseIntError;
use std::str::FromStr;
use std::path::Path;

fn main() {
    let input_file_path = Path::new("../input");
    let input = input::read_as_string(input_file_path)
        .expect("could not read input file")
        .strip_suffix('\n')
        .unwrap()
        .to_string();

    println!("{:?}", input);

    let initial_state = School::from_str(&input).expect("could not parse input data");
    let final_state = initial_state.step_by(80);

    println!("After 80 days: {} fish:", final_state.count());

    println!("After 256 days: {} fish:", initial_state.step_by(256).count());
    // println!("{}", final_state);
}


#[derive(Debug, PartialEq, Clone)]
struct School {
    fish: Vec<u32>,
}

impl fmt::Display for School {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.fish)
    }
}

impl FromStr for School {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let fish: Result<Vec<_>, Self::Err> =
            s.split(',').map(|ch| ch.parse()).collect();
        match fish {
            Ok(fish) => Ok(Self { fish }),
            Err(e) => Err(e),
        }
    }
}

impl School {
    fn next_day(&self) -> Self {
        let birthing_count = self.fish.iter().filter(|a| **a == 0).count();
        let baby_fish = vec![8; birthing_count];

        let current_fish = self
            .fish
            .iter()
            .map(|a| match a {
                0 => 6,
                _ => a - 1,
            })
            .collect::<Vec<_>>();
        let next_school = [&current_fish[..], &baby_fish].concat();
        Self { fish: next_school }
    }

    fn step_by(&self, days: usize) -> Self {
        (0..days).fold(self.clone(), |school, _| school.next_day())
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

        assert_eq!(
            School::from_str("6,8").unwrap(),
            school.step_by(5)
        );
    }

    #[test]
    fn example() {
        let input = "3,4,3,1,2".to_string();
        let initial_state = School::from_str(&input).unwrap();

        let final_state = initial_state.step_by(18);

        assert_eq!(
            School::from_str(
                "6,0,6,4,5,6,0,1,1,2,6,0,1,1,1,2,2,3,3,4,6,7,8,8,8,8"
            ).unwrap(),
            final_state
        );
        // println!("{}", final_state);
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
