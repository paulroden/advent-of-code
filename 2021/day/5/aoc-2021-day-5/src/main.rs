use regex::{self, Regex};
use std::path::Path;
use std::str::FromStr;
use std::cmp::Ordering;
use std::collections::HashMap;
use lazy_static::lazy_static;

fn main() {
    let input_file_path = Path::new("../input");
    let parsed_lines = input::read_lines(
        input_file_path,
        |line| Ok(LineVector::from_str(line).unwrap())
    )
    .expect("could not read input file");

    let non_diagonals = parsed_lines.iter()
        .filter(|line| !line.is_diagonal())
        .copied()
        .collect::<Vec<_>>();
        
    let grid = SparseGrid::from_lines(&non_diagonals);

    println!("{}", grid.count_points_above(2));

}


fn parse_captures_to_i32(captures: &regex::Captures, i: usize) -> i32 {
    captures.get(i).unwrap().as_str().parse::<i32>().unwrap()
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct LineVector {
    start: [i32; 2],
    end: [i32; 2],
}

impl LineVector {
    fn new(start: (i32, i32), end: (i32, i32)) -> Self {
        let start = [start.0, start.1];
        let end = [end.0, end.1];
        Self { start, end }
    }
    
    fn is_diagonal(&self) -> bool {
        self.start[0] != self.end[0] && self.start[1] != self.end[1]
    }

    // returns a version of the co-ordinate pair where the `start` is
    // the 'smaller' point and `end` is the 'larger' one
    // such that (x1 <= x2), (y1 <= y2)
    fn ordered(&self) -> Self {
        if self.start < self.end {
            *self
        } else {
            Self {
                start: self.end,
                end: self.start
            }
        }
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
            let start = [
                parse_captures_to_i32(&captures, 1),
                parse_captures_to_i32(&captures, 2),
            ];
            let end = [
                parse_captures_to_i32(&captures, 3),
                parse_captures_to_i32(&captures, 4),
            ];
            Ok(Self { start, end })
        } else {
            Err(LineParseError)
        }
    }
}

#[derive(Debug)]
struct SparseGrid {
    points: HashMap<(i32, i32), i32>
}

impl SparseGrid {
    fn from_lines(lines: &[LineVector]) -> Self {
        let points = lines.iter().fold(HashMap::new(), | mut map, line | {
            let line = line.ordered();
            for x in line.start[0] ..= line.end[0] {
                for y in line.start[1] ..= line.end[1] {
                    *map.entry((x, y)).or_insert(0) += 1;
                }
            }
            map
        });
        Self { points }
    }

    fn count_points_above(&self, x: i32) -> usize {
        self.points.values()
        .filter(|v| **v >= x)
        .count() 
    }
}




mod tests {
    use std::str::FromStr;
    use crate::*;

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
            [565,190]
        );
        assert_eq!(
            parsed.last().unwrap().end,
            [98,844]
        );
    }

    #[test]
    fn recognised_diagonal_line() {
        let line = LineVector::new( (973,82), (308,747) );
        assert!(line.is_diagonal());
    }

    #[test]
    fn example() {
        use std::collections::HashMap;

        let sample_lines = "0,9 -> 5,9\n8,0 -> 0,8\n9,4 -> 3,4\n2,2 -> 2,1\n7,0 -> 7,4\n6,4 -> 2,0\n0,9 -> 2,9\n3,4 -> 1,4\n0,0 -> 8,8\n5,5 -> 8,2";
        let parsed_lines = sample_lines.lines().map(|line| LineVector::from_str(line).unwrap());

        let non_diagonals = parsed_lines.filter(|line| !line.is_diagonal()).collect::<Vec<_>>();
        let grid = SparseGrid::from_lines(&non_diagonals);
        
        assert_eq!(non_diagonals.len(), 6);
        assert_eq!(grid.count_points_above(2), 5);
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
