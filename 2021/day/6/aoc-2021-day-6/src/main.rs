use std::fmt;
use std::num::ParseIntError;
use std::path::Path;
use std::str::FromStr;

fn main() {
    let input_file_path = Path::new("../input");
    let input = input::read_as_string(input_file_path)
        .expect("could not read input file")
        .strip_suffix('\n')
        .unwrap()
        .to_string();

    println!("{:?}", input);

    let initial_state = School::from_str(&input).expect("could not parse input data");
    // let part_1_state = initial_state.step_by(80);
    println!("After 80 days: {}", initial_state.clone().step_by(80).count());
    // let part_2_state = &initial_state.step_by(256);
    println!("After 256 days: {}", initial_state.clone().step_by(256).count());

    // // Part 1
    // let final_state = initial_state.step_by(80);
    // println!("After 80 days: {} fish.", final_state.count());

    // // Part 2
    // let final_state_2 = initial_state.step_by(256);
    // println!("After 256 days: {} fish.", final_state_2.count());
    // // println!("{}", final_state);
}

#[derive(Debug, PartialEq, Clone)]
struct School {
    fish: [u64; 9],
}

impl fmt::Display for School {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.fish.iter().enumerate().collect::<Vec<_>>())
    }
}

impl FromStr for School {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, ParseIntError> {
        let all_fish: Vec<u32> = s
            .split(',')
            .map(|ch| ch.parse().expect("Could not parse to int."))
            .collect::<Vec<_>>();

        let mut fish = [0; 9];

        for time in all_fish {
            fish[time as usize + 1] += 1;
        }
        Ok(Self { fish })
    }
}

impl School {
    fn next_day(&self) -> Self {
        // let mut current = self.fish.clone();
        let fish = [
            self.fish[1],                   // 0
            self.fish[2],                   // 1
            self.fish[3],                   // 2
            self.fish[4],                   // 3
            self.fish[5],                   // 4
            self.fish[6],                   // 5
            self.fish[7] + self.fish[0],    // 6
            self.fish[8],                   // 7
            self.fish[0],                   // 8
        ];
        Self { fish }
  }

    fn step_by(self, days: usize) -> Self {
        (0..=days).fold(self, |school, _| school.next_day())
    }

    fn count(&self) -> u64 {
        self.fish.iter().sum()
    }
}

mod tests {
    use crate::*;
    #[test]
    fn single_fish() {
        let school = School::from_str("4").unwrap();

        assert_eq!(School::from_str("6,8").unwrap(), school.step_by(5));
    }

    #[test]
    fn example() {
        let input = "3,4,3,1,2".to_string();
        let initial_state = School::from_str(&input).unwrap();

        let mut step = initial_state.clone();
        for i in 0..18 {
            step = step.step_by(1);
            println!("{}: {}", i, step);
        }

        let final_state = initial_state.step_by(18);

        assert_eq!(
            5934,
            final_state.count()
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
