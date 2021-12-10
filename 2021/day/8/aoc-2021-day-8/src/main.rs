fn main() {
    use std::path::Path;
    use ndarray::prelude::*;

    let input_file_path = Path::new("../input");
    let data = input::read_as_string(input_file_path).expect("Could not read input file");
    let heights = input::read_char_array(&data);

    let mut count = 0;
    let i_max: usize = heights.ncols();
    let j_max: usize = heights.nrows();
    for j in 0..j_max {
        for i in 0..i_max {
            
            let view = heights.slice(s![
                j.saturating_sub(1)..=(j + 1).min(j_max - 1),
                i.saturating_sub(1)..=(i + 1).min(i_max - 1),
            ]);
            let e = heights.get((j, i));
            let minimum = view.iter().min();
            match e == minimum {
                true => {
                    if let Some(value) = e {
                        count += value + 1;
                    }
                    println!("{:?} in:\n{:?}\n", e, view)
                },
                false => (),
            }
        }
    }
    println!("Risk Count: {}", count);
}

mod tests {

    #[test]
    fn parses_input_file() {
        use crate::input::read_as_string;
        use std::path::Path;

        let input_file_path = Path::new("../input");

        let data = read_as_string(input_file_path).unwrap();
        let n_cols = data.lines().next().unwrap().len();
        let n_rows = data.lines().count();

        assert_eq!(n_cols, 100);
        assert_eq!(n_rows, 100);
        println!("{} rows × {} columns", n_rows, n_cols);
    }

    #[test]
    fn string_of_numerical_chars_to_ndarray() {
        use ndarray::prelude::*;
        use crate::input;

        let chars = "\
        986545679234\n\
        299435989015\n\
        398929899123\n\
        987898788934\n\
        876685667895\n\
        654534458896\n\
        743212345789\n\
        894323456899\n\
        965444879998\n\
        878656989349\n\
        ";


        let ary = input::read_char_array(chars);
        println!("{:?}", ary);
    }
    #[test]
    fn example_part_1() {
        use ndarray::prelude::*;

        let heights = arr2(&[
            [2, 1, 9, 9, 9, 4, 3, 2, 1, 0],
            [3, 9, 8, 7, 8, 9, 4, 9, 2, 1],
            [9, 8, 5, 6, 7, 8, 9, 8, 9, 2],
            [8, 7, 6, 7, 8, 9, 6, 7, 8, 9],
            [9, 8, 9, 9, 9, 6, 5, 6, 7, 8],
        ]);

        // println!("{:?}", heights);
        // println!("{:?} == {}, {}", heights.shape(), heights.nrows(), heights.ncols());
        // println!("{:?}", heights.get((0,0)));
        // println!("{:?}", heights.get((3,3)));
        // let mut count = 0_usize;
        let i_max: usize = heights.ncols();
        let j_max: usize = heights.nrows();
        for j in 0..j_max {
            for i in 0..i_max {
                
                let view = heights.slice(s![
                    j.saturating_sub(1)..=(j + 1).min(j_max - 1),
                    i.saturating_sub(1)..=(i + 1).min(i_max - 1),
                ]);
                let e = heights.get((j, i));
                let minimum = view.iter().min();
                match e == minimum {
                    true => println!("{:?} in:\n{:?}\n", e, view),
                    false => (),
                }

                // let mask_template = arr2(&[[0, 1, 0], [1, 1, 1], [0, 1, 0]]);
                // let mask = mask_template.slice(
                //     s![
                //         (-1) ..=
                //     ]
                // )

                // let kernel = mask * view;
                // println!("{:?}", kernel);

                // count += 1;
                // println!(
                //     "count: {}, value: {:?} ",
                //     count,
                //     heights.get((j,i))
                // );
            }
        }
    }

    #[test]
    fn laplace() {
        use ndarray::prelude::*;

        let heights = arr2(&[
            [2, 1, 9, 9, 9, 4, 3, 2, 1, 0],
            [3, 9, 8, 7, 8, 9, 4, 9, 2, 1],
            [9, 8, 5, 6, 7, 8, 9, 8, 9, 2],
            [8, 7, 6, 7, 8, 9, 6, 7, 8, 9],
            [9, 8, 9, 9, 9, 6, 5, 6, 7, 8],
        ]);
        // let mask = s![0..4, 0..3];
        // let view = heights.slice(mask);

        for ((i,j), v) in heights.indexed_iter() {
            println!("({},{})", i, j);
            
            let slice = s![
                i.saturating_sub(1) .. (i + 2).min(heights.nrows()),
                j.saturating_sub(1) .. (j + 2).min(heights.ncols()),
            ];
            let mask = arr2(&[
                [0, 1, 0],
                [1, 1, 1],
                [0, 1, 0],
            ]);
            let m_i_min = match i.saturating_sub(1) {
                0 => 1,
                _ => 0,
            };
            let m_j_min = match j.saturating_sub(1) {
                0 => 1,
                _ => 0,
            };
            let m_i_max = match (i + 2) > heights.nrows() {
                true => mask.nrows() - 1,
                false => mask.nrows()
            };
            let m_j_max = match (j + 2) > heights.ncols() {
                true => mask.ncols() - 1,
                false => mask.ncols()
            };
            let m_slice = s![
                m_i_min .. m_i_max,
                m_j_min .. m_j_max
            ];
            //   |
            // -------
            // 
            // println!("{:?}", slice);
            // println!("{:?}", m_slice);

            let view = heights.slice(slice);
            let kernel = mask.slice(m_slice);

            let minimum = view.iter().min();
            match Some(v) == minimum {
                true => println!("*{:?}* in:\n{:?}\n{:?}\n", v, view, kernel),
                false => println!("not {:?} in:\n{:?}\n{:?}\n", v, view, kernel),
            }
        }

        // for (i, chunk) in heights.exact_chunks((3, 3)).into_iter().enumerate() {
        //     println!("i: {}\n {:?}", i, chunk);
        // }

        
        // fn laplacian(v: &ArrayView2<f32>) -> Array2<f32> {
        //     -4. * &v.slice(s![1..-1, 1..-1])
        //     + v.slice(s![ ..-2, 1..-1])
        //     + v.slice(s![1..-1,  ..-2])
        //     + v.slice(s![1..-1, 2..  ])
        //     + v.slice(s![2..  , 1..-1])
        // }
    }
}

mod input {
    use std::fs::File;
    use std::io::{self, prelude::*, BufReader};
    use std::num::ParseIntError;
    use std::path::Path;
    use ndarray::Array2;
    

    pub fn read_as_string(path: &Path) -> Result<String, io::Error> {
        let mut file = File::open(path)?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;
        Ok(buffer)
    }

    pub fn read_char_array(s: &str) -> Array2<i32> {
        let n_cols = s.lines().next().unwrap().len();
        let n_rows = s.lines().count();

        let mut array = Array2::<i32>::zeros((n_rows, n_cols));
        s.lines()
            .flat_map(|line| line.chars())
            .collect::<String>()
            .chars()
            .zip(array.iter_mut())
            .for_each(|(ch, e)| {
                if let '0'..='9' = ch {
                    *e = ch.to_digit(10).unwrap() as i32
                }
            });
        array
    }
}
