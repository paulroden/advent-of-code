use std::ops::ControlFlow;
use std::path::Path;

fn main() {
    let input_file_path = Path::new("../input");
    let file_by_lines = input::read_lines(input_file_path, |line| Ok(line.to_string()))
        .expect("could not read input file");

    let bit_awway =
        BitArray::from_lines(&file_by_lines).expect("Could not parse text lines to bit array.");

    // # Part 1
    let commons = (0..bit_awway.width)
        .map(|i| commonest_bit(&bit_awway.column(i), true))
        .collect::<Vec<_>>();

    let base_10 = bits_to_usize(&commons);
    let gamma_rate = base_10;
    let epsilon_rate = !gamma_rate & bit_mask(bit_awway.width);

    println!("ɣ: {:#016b}, {}", gamma_rate, gamma_rate);
    println!("ε: {:#016b}, {}", epsilon_rate, epsilon_rate);
    println!("ɣ × ε => {}\n", gamma_rate * epsilon_rate);
    assert_eq!(gamma_rate * epsilon_rate, 2261546);

    // # Part 2
    // ## Oxygen Generator
    let oxygen_generator_bits =
        match (0..bit_awway.width).try_fold(bit_awway.clone(), |mut ba, pos| {
            let desired_bit = commonest_bit(&ba.column(pos), true);
            ba = ba.filter(pos, desired_bit);
            if ba.length == 1 {
                ControlFlow::Break(ba)
            } else {
                ControlFlow::Continue(ba)
            }
        }) {
            ControlFlow::Break(bits) => bits,
            ControlFlow::Continue(bits) => bits,
        };

    let oxygen_generator = bits_to_usize(oxygen_generator_bits.row(0));
    println!("O₂ Gen: {} == {:#016b}", oxygen_generator, oxygen_generator);

    // ## CO2 Scrubber
    let co2_scrubber_bits = match (0..bit_awway.width).try_fold(bit_awway.clone(), |mut ba, pos| {
        let desired_bit = !commonest_bit(&ba.column(pos), true);
        ba = ba.filter(pos, desired_bit);
        if ba.length == 1 {
            ControlFlow::Break(ba)
        } else {
            ControlFlow::Continue(ba)
        }
    }) {
        ControlFlow::Break(bits) => bits,
        ControlFlow::Continue(bits) => bits,
    };

    let co2_scrubber = bits_to_usize(co2_scrubber_bits.row(0));
    println!("CO₂ Scrub.: {} == {:#016b}", co2_scrubber, co2_scrubber);

    println!("Part 2 Result: {}", oxygen_generator * co2_scrubber);
}

#[derive(Debug, Clone)]
struct BitArray {
    width: usize,
    length: usize,
    bits: Vec<bool>,
}

impl BitArray {
    fn from_lines(lines: &[String]) -> Result<Self, String> {
        let width = lines.first().unwrap().chars().count();
        let length = lines.len();

        let parsed_bits: Result<Vec<bool>, _> = lines
            .iter()
            .flat_map(|line| line.chars().map(parse_bit))
            .collect();
        match parsed_bits {
            Ok(bits) => Ok(Self {
                width,
                length,
                bits,
            }),
            Err(e) => Err(e),
        }
    }

    fn row(&self, i: usize) -> &[bool] {
        let from = i * self.width;
        let to = (i + 1) * self.width;
        &self.bits[from..to]
    }

    fn column(&self, i: usize) -> Vec<bool> {
        self.bits
            .iter()
            .skip(i)
            .step_by(self.width)
            .copied()
            .collect()
    }

    fn filter(&self, bit_index: usize, value: bool) -> Self {
        let filtered = (0..self.length)
            .map(|i| self.row(i))
            .filter(|r| r[bit_index] == value)
            .flatten()
            .copied()
            .collect::<Vec<_>>();
        Self {
            width: self.width,
            length: filtered.len() / self.width,
            bits: filtered,
        }
    }
}

pub fn parse_bit(ch: char) -> Result<bool, String> {
    match ch {
        '0' => Ok(false),
        '1' => Ok(true),
        other => Err(format!("could not parse {}", other)),
    }
}

pub fn count_ones(bits: &[bool]) -> usize {
    bits.iter().filter(|b| **b).count()
}

pub fn count_zeros(bits: &[bool]) -> usize {
    bits.len() - count_ones(bits)
}

pub fn commonest_bit(bits: &[bool], if_equal: bool) -> bool {
    let most_common = (2 * count_ones(bits)).cmp(&bits.len());
    match most_common {
        std::cmp::Ordering::Greater => true, // more ones than zeroes
        std::cmp::Ordering::Less => false,   // fewer ones than zeroes
        std::cmp::Ordering::Equal => if_equal,
    }
}

pub fn bits_to_usize(bits: &[bool]) -> usize {
    bits.iter()
        .rev()
        .enumerate()
        .fold(0, |n, (i, b)| n | (*b as usize) << i)
}

pub fn bit_mask(n_bits: usize) -> usize {
    (0..n_bits).fold(1, |m, _| m << 0b1) - 1
}


#[allow(unused_imports)]
mod tests {
    use crate::*;
    use std::io::BufRead;

    #[test]
    fn matches_supplied_example_part_1() {
        let sample_data =
            "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010\n";
        let sample_data_lines = sample_data
            .lines()
            .map(|line| line.to_string())
            .collect::<Vec<String>>();
        let sample_bit_array = BitArray::from_lines(&sample_data_lines).unwrap();
        let commons = (0..sample_bit_array.width)
            .map(|i| commonest_bit(&sample_bit_array.column(i), true))
            .collect::<Vec<_>>();

        let base_10 = bits_to_usize(&commons);
        let gamma_rate = base_10;
        let epsilon_rate = !gamma_rate & bit_mask(sample_bit_array.width);
        assert_eq!(gamma_rate, 22);
        assert_eq!(epsilon_rate, 9);

        // O2 Generator

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
