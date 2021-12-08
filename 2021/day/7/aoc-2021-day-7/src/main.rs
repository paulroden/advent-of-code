use std::collections::HashMap;
use std::path::Path;

fn main() {
    let input_file_path = Path::new("../input");
    let input = input::read_lists(input_file_path, ",", |ch| Ok(ch.parse::<i32>().unwrap()))
        .expect("Coulr not read input file.");

    let positions = Positions::from_list(&input[0]);

    let metric_part_1 = |x: i32, y: i32| ((x - y) as i32).abs();
    println!("{:?}", positions.minimum_displacements(metric_part_1));

    let metric_part_2 = |x: i32, y: i32| {
        let d = ((x - y) as i32).abs();
        d * (d + 1) / 2
    };
    println!("{:?}", positions.minimum_displacements(metric_part_2));
}

#[derive(Debug)]
struct Positions {
    counts: HashMap<i32, i32>,
}

impl Positions {
    fn from_list(list: &[i32]) -> Self {
        let counts = list.iter().fold(HashMap::new(), |mut map, &k| {
            *map.entry(k).or_insert(0) += 1;
            map
        });
        Self { counts }
    }

    fn unique(&self) -> Vec<i32> {
        self.counts.keys().copied().collect()
    }

    fn weights(&self) -> Vec<i32> {
        self.counts.values().copied().collect()
    }

    fn total_elements(&self) -> i32 {
        self.counts.values().sum()
    }

    fn minimum_displacements(&self, distance_metric: fn(i32, i32) -> i32) -> Option<i32> {
        let xs = self.unique();
        let ws = self.weights();
        match (xs.iter().min(), xs.iter().max()) {
            (Some(min_x), Some(max_x)) => (*min_x..=*max_x)
                .map(|y| {
                    xs.iter()
                        .zip(ws.iter())
                        .map(|(x, w)| w * distance_metric(*x, y))
                        .sum()
                })
                .min(),
            (_, _) => None,
        }
    }
}

mod tests {
    #[test]
    fn example() {
        use crate::Positions;

        let example_input = vec![16,1,2,0,4,2,7,1,2,14];
        let positions = Positions::from_list(&example_input);

        let metric_part_1 = |x: i32, y: i32| ((x - y) as i32).abs();
        assert_eq!(
            Some(37),
            positions.minimum_displacements(metric_part_1)
        );

        let metric_part_2 = |x: i32, y: i32| {
            let d = ((x - y) as i32).abs();
            (0..=d).sum()
        };
        assert_eq!(
            Some(168),
            positions.minimum_displacements(metric_part_2)
        );
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
