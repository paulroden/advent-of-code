use std::num::ParseIntError;
use std::path::Path;
use std::collections::HashMap;

fn main() {
    let input_file_path = Path::new("../input");
    let input = input::read_lists(
        input_file_path,
        ",",
        |ch| Ok(ch.parse::<i32>().unwrap())
    ).expect("Coulr not read input file.");

    let positions = Positions::from_list(&input[0]);

    println!("{:?}", positions.total_elements());
}

#[derive(Debug)]
struct Positions {
    counts: HashMap<i32, u32>
}

impl Positions {
    fn from_list(list: &[i32]) -> Self {
        let counts = list.iter().fold(HashMap::new(), | mut map, &k| {
            *map.entry(k).or_insert(0) += 1;
            map
        });
        Self { counts }
    }

    fn total_elements(&self) -> u32 {
        self.counts.values().sum()
    }
}


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

    pub fn read_lists<T, P>(
        path: &Path,
        delimiter: &str,
        element_parser: P,
    ) -> Result<Vec<Vec<T>>, io::Error>
    where
        P: Fn(&str) -> Result<T, ParseError>,
        T: std::str::FromStr,
    {
        let file = File::open(path)?;
        let buffer = BufReader::new(file);

        let lists = buffer
            .lines()
            .map(|line| {
                line.unwrap()
                    .split(delimiter)
                    .map(|ch| element_parser(ch).unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        Ok(lists)
    }
}
