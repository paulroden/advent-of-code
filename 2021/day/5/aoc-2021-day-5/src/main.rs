use regex::{self, Regex};
use std::path::Path;
use std::str::FromStr;
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

    fn has_unit_gradient(&self) -> bool {
        match self.is_diagonal() {
            false => false,
            true =>  {
                let gradient = (self.end[0] - self.start[0]) / (self.end[1] - self.start[1]);
                gradient.abs() == 1
            },
        }
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

    fn gradient_0(&self) -> i32 {
        let d0 = self.end[0] - self.start[0];
        let d1 = self.end[1] - self.start[1];
        match d0 {
            0 => 0,
            _ => d1 / d0
        }
    }
    fn gradient(&self) -> [[i32; 2]; 2] {
        let d0 = self.end[0] - self.start[0];
        let d1 = self.end[1] - self.start[1];
        match (d0, d1) {
            (0, 0) => [[0,0], [0, 0]],
            (0, _) => [[0, d0 / d1], [0, 1]],
            (_, 0) => [[1, 0], [d1 / d0, 0]],
            _ => [[1, d0 / d1], [d1 / d0, 1]]
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
            let grad = line.gradient();
            for i in 0..=(line.end[0] - line.start[0])  {
                for j in 0..=(line.end[1] - line.start[1]) {
                    let x = (i * grad[0][0]) + (j * grad[0][1]) + line.start[0];
                    let y = (i * grad[1][0]) + (j * grad[1][1]) + line.start[1];
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

    fn bounds(&self) -> [(i32, i32); 2] {
        let min_bound = (
            self.points.keys().map(|(x, _)| *x).min().unwrap(),
            self.points.keys().map(|(_, y)| *y).min().unwrap(),
        );
        let max_bound = (
            self.points.keys().map(|(x, _)| *x).max().unwrap(),
            self.points.keys().map(|(_, y)| *y).max().unwrap(),
        );
        [min_bound, max_bound]
    }

    fn to_dense(&self) -> Vec<Vec<i32>> {
        let bounds = self.bounds();
        let mut dense_grid: Vec<Vec<i32>> =
            (bounds[0].0 ..= bounds[1].1).map(|_|
                (bounds[0].0 ..= bounds[1].0)
                .map(|_| 0)
                .collect::<Vec<_>>()
            )
        .collect();

        self.points.iter().for_each(|((j,i), v)|
            dense_grid[*i as usize][*j as usize] = *v
        );

        dense_grid
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
    fn gradients() {
        let flat_0 = LineVector::new((2,2), (2,5));
        let flat_1 = LineVector::new((2,2), (5,2));
        let diag_01 = LineVector::new((2,2), (5,5));

        assert_eq!([[0, 0], [0, 1]], flat_0.gradient());
        assert_eq!([[1, 0], [0, 0]], flat_1.gradient());
        assert_eq!([[1, 1], [1, 1]], diag_01.gradient());
    }

    #[test]
    fn example_part_1() {
        let sample_lines = "0,9 -> 5,9\n8,0 -> 0,8\n9,4 -> 3,4\n2,2 -> 2,1\n7,0 -> 7,4\n6,4 -> 2,0\n0,9 -> 2,9\n3,4 -> 1,4\n0,0 -> 8,8\n5,5 -> 8,2";
        let parsed_lines = sample_lines.lines().map(|line| LineVector::from_str(line).unwrap());

        let non_diagonals = parsed_lines.filter(|line| !line.is_diagonal()).collect::<Vec<_>>();
        let grid = SparseGrid::from_lines(&non_diagonals);
        
        // assert_eq!(1,2);
        for row in &grid.to_dense() {
            println!("{:?}", row);
        }

        assert_eq!(non_diagonals.len(), 6);
        assert_eq!(grid.count_points_above(2), 5);   
    }

    #[test]
    fn example_part_2() {
        let sample_lines = "0,9 -> 5,9\n8,0 -> 0,8\n9,4 -> 3,4\n2,2 -> 2,1\n7,0 -> 7,4\n6,4 -> 2,0\n0,9 -> 2,9\n3,4 -> 1,4\n0,0 -> 8,8\n5,5 -> 8,2";
        let parsed_lines = sample_lines.lines().map(|line| LineVector::from_str(line).unwrap());

        let valid_lines = parsed_lines.filter(|line|
            !line.is_diagonal() || line.has_unit_gradient()
        )
        .collect::<Vec<_>>();
        let grid = SparseGrid::from_lines(&valid_lines);
        
        println!("{:?}", grid);

        assert_eq!(valid_lines.len(), 10);
        assert_eq!(grid.count_points_above(2), 12);
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
