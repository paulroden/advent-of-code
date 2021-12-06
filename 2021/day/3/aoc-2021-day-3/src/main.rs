use std::path::Path;

fn main() {
    let input_file_path = Path::new("../input");
    let file_by_lines = input::read_lines(input_file_path, |line| Ok(line.to_string()))
        .expect("could not read input file");

    let bit_awway =
        BitArray::from_lines(&file_by_lines).expect("Could not parse text lines to bit array.");

    let commons = (0..bit_awway.width)
        .map(|i| commonest_bit(&bit_awway.column(i)))
        .collect::<Vec<_>>();

    let base_10 = bits_to_usize(&commons);
    let gamma_rate = base_10;
    let epsilon_rate = !gamma_rate & bit_mask(bit_awway.width);

    println!("{:#016b}, {}", gamma_rate, gamma_rate);
    println!("{:#016b}, {}", epsilon_rate, epsilon_rate);
    println!("{}", gamma_rate * epsilon_rate);
    assert_eq!(gamma_rate * epsilon_rate, 2261546);
}


#[derive(Debug)]
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

pub fn commonest_bit(bits: &[bool]) -> bool {
    count_ones(bits) >= (bits.len() / 2)
}

pub fn bits_to_usize(bits: &[bool]) -> usize {
    bits.iter()
        .rev()
        .enumerate()
        .fold(0, |n, (i,b)|
            n | (*b as usize) << i
        )
}

pub fn bit_mask(n_bits: usize) -> usize {
    (0..n_bits).fold(1, |m, _| m << 0b1) - 1
}


pub fn count_lines_of_bits(
    lines: impl Iterator<Item = std::io::Result<String>>,
    line_width: usize,
) -> (usize, Vec<usize>) {
    lines
        .enumerate()
        .fold((0, vec![0; line_width]), |(_, bits), (i, line)| {
            (i + 1, {
                bits.iter()
                    .zip(line.unwrap().chars())
                    .map(|(bit, ch)| {
                        bit + match ch {
                            '1' => 1,
                            _ => 0,
                        }
                    })
                    .collect()
            })
        })
}

pub fn common_bits((line_count, bits): &(usize, Vec<usize>)) -> Vec<usize> {
    let half_count = *line_count / 2;

    bits.iter()
        .map(|b| if *b >= half_count { 1 } else { 0 })
        .collect()
}

pub fn bit_vec_to_unsigned_int(bits: &[usize]) -> usize {
    bits.iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (b, p)| acc + (p * 2_usize.pow(b as u32)))
}

pub fn infer_bit_width(input: &[u8]) -> usize {
    input.iter().take_while(|b| **b != b'\n').cloned().count()
}

pub fn split_lines_by_first_bit(
    lines: impl Iterator<Item = std::io::Result<String>>,
    // line_width: usize,
) -> (Vec<String>, Vec<String>) {
    let mut zeros = Vec::new();
    let mut ones = Vec::new();
    for line in lines.flatten() {
        match line.chars().next() {
            Some('0') => zeros.push(line),
            Some('1') => ones.push(line),
            _ => (),
        }
    }
    (zeros, ones)
}

#[allow(unused_imports)]
mod tests {
    use crate::*;
    use std::io::BufRead;
    #[test]
    fn infers_number_of_bits_per_line() {
        assert_eq!(5, infer_bit_width(b"00100\n11110\n"));
        assert_eq!(
            12,
            infer_bit_width(b"101000111100\n000011111101\n011100000100\n")
        );
    }

    #[test]
    fn matches_supplied_example() {
        let sample_input =
            b"00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010\n";
        let line_width = 5;
        let position_counts = count_lines_of_bits(sample_input.lines(), line_width);

        let base_10 = bit_vec_to_unsigned_int(&common_bits(&position_counts));
        let mask = 2_usize.pow(line_width.try_into().unwrap()) - 1;
        let gamma_rate = base_10;
        let epsilon_rate = !gamma_rate & mask;
        assert_eq!(gamma_rate, 22);
        assert_eq!(epsilon_rate, 9);
    }

    #[test]
    fn accumulates_lines_of_bits() {
        let sample =
            b"101000111100\n000011111101\n011100000100\n100100010000\n011110010100\n101001100000\n";
        let line_width = 12;

        let position_counts = count_lines_of_bits(sample.lines(), line_width);

        assert_eq!(
            (6, vec![3, 2, 4, 3, 2, 2, 3, 4, 2, 4, 0, 1]),
            position_counts
        )
    }

    #[test]
    fn computes_most_common_bits() {
        let most_common = common_bits(&(6, vec![3, 2, 4, 3, 2, 2, 3, 4, 2, 4, 0, 1]));

        assert_eq!(most_common, vec![1, 0, 1, 1, 0, 0, 1, 1, 0, 1, 0, 0])
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
