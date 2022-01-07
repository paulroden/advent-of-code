use ndarray::prelude::*;
use num_traits::Zero;
use std::path::Path;

fn main() {
    let input_file_path = Path::new("../input");
    let data = input::read_as_string(input_file_path).expect("Could not read input file");
    let heights = input::read_char_array(&data);

    // Part 1
    let count = heights.indexed_iter().fold(0, |count, ((i, j), value)| {
        let view = heights.slice(s![
            i.saturating_sub(1)..=(i + 1).min(heights.ncols() - 1),
            j.saturating_sub(1)..=(j + 1).min(heights.nrows() - 1),
        ]);
        match Some(value) == view.iter().min() {
            true => count + value + 1,
            false => count,
        }
    });
    println!("Risk Count: {}", count);

}

// neighbours: start from cell above centre point,
// i.e. (-1,0) from centre; walk clockwise through elements
// including initial element once more.
// e.g.:
//      █▒█
//      ▒██
//      ██▒
// true => boundary pixel ▒
// false => other pixel █


// encode modular phases iterating over adjacent cells in a clockwise direction
// equivalent to taking the real part of $i^k$ for $k in {0,1,2,3}$.
fn phase(k: usize) -> i32 {
    match k % 4 {
        0 => -1,
        1 => 0,
        2 => 1,
        3 => 0,
        _ => -1, // can't get here
    }
}

// 
// 
fn neighbours(
    stencil: &Array2<Option<i32>>,
    centre: &[i32; 2],
) -> Vec<i32> {
    (0..4)
    .flat_map(|k| {
        // `ac` stencil-local co-ordinate for cell adjacent to
        // centre in dimension 0
        let ac = [
            (centre[0] + phase(k)) as usize,
            (centre[1] + phase(k + 1)) as usize,
        ];
        // `bc` stencil-local co-ordinate for cell adjacent to
        // centre in dimension 1
        let bc = [
            (centre[0] + phase(k + 1)) as usize,
            (centre[1] + phase(k + 2)) as usize,
        ];
        // diagonal cell of (a,b) -> d
        let dc = [ac[k % 2], bc[(k + 1) % 2]];
        
        let a = stencil.get(ac);
        let b = stencil.get(bc);
        let d = if a == Some(&None) && b == Some(&None) {
            None
        } else {
            stencil.get(dc)
        };
        [a, b, d]
    })
    .flatten() // flatten the 4 lists of neghbours
    .flatten() // remove any `None`s from flat list (<=> cells on or beyond the boundary)
    .copied()
    .collect()
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
        use crate::input;
        use ndarray::prelude::*;

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
    fn example_part_2() {
        use ndarray::prelude::*;

        let heights = arr2(&[
            [2, 1, 9, 9, 9, 4, 3, 2, 1, 0],
            [3, 9, 8, 7, 8, 9, 4, 9, 2, 1],
            [9, 8, 5, 6, 7, 8, 9, 8, 9, 2],
            [8, 7, 6, 7, 8, 9, 6, 7, 8, 9],
            [9, 8, 9, 9, 9, 6, 5, 6, 7, 8],
        ]);

        println!("{:?}", heights);
        let boundary_value = 9;
        // let boundaries = heights.mapv(|v| (v == boundary_value) as usize);
        // println!("{:?}", boundaries);

        let mut regions = heights.mapv(|v| match v == boundary_value {
            true => None,
            false => Some(0),
        });
        println!("{:?}", regions);

        // cases:
        //  - all adjacent cells are boundary cells:
        //    cell is a singleton => increment counter and write to cell
        //  - cell is surrounded by some mix of boundary/non-boundary cells:
        //    check if any 'neighbouring cells' are non-boundary have marked value,
        //      if so, mark cell with the max. of this value (there should only be one)
        //      if no such value exists, increment `counter` and write this updated value to the cell.
        //  where 'neighbouring cells' are
        //        { [i, j-1], [i-1, j], [i, j+1], [] }  if any such exist and are not boundary cells
        let mut counter: usize = 1;

        for i in 0..regions.nrows() {
            for j in 0..regions.ncols() {
                let cell_value = regions.get((i, j)).unwrap();
                match cell_value {
                    None => (),
                    // get adjacent cells from `regions` and check if any have already been filled
                    Some(_) => {
                        let prev_j = match j == 0 {
                            true => None,
                            false => unsafe { regions.uget((i, j - 1)).as_ref() },
                        };
                        let next_j = match j < regions.ncols() - 2 {
                            true => unsafe { regions.uget((i, j + 1)).as_ref() },
                            false => None,
                        };
                        let prev_i = match i == 0 {
                            true => None,
                            false => unsafe { regions.uget((i - 1, j)).as_ref() },
                        };
                        let next_i = match i < regions.nrows() - 2 {
                            true => unsafe { regions.uget((i + 1, j)).as_ref() },
                            false => None,
                        };

                        let adjacents = [prev_j, prev_i, next_j, next_i]
                            .iter()
                            .copied()
                            .to_owned()
                            .collect::<Vec<_>>();

                        match adjacents.iter().copied().flatten().max() {
                            // all adjacents are None => boundary cells
                            None => {
                                counter += 1;
                                let cell = unsafe { regions.uget_mut((i, j)) };
                                *cell = Some(counter);
                            }
                            // one or more of the adjacent cells is not a boundary cell
                            Some(&region_index) => {
                                // if the max value here is 0, all non-boundary adjacent cells
                                // are untouched, so need a new region index value
                                if region_index == 0 {
                                    counter += 1;
                                    let cell = unsafe { regions.uget_mut((i, j)) };
                                    *cell = Some(counter);
                                // otherwise one of the adjacent cells belongs to a region
                                // which has already been given an index value. Copy it to
                                // this cell
                                } else {
                                    let cell = unsafe { regions.uget_mut((i, j)) };
                                    *cell = Some(region_index);
                                }
                            }
                        }
                    }
                }
            }
        }

        let regions_unwrapped = regions.mapv(|v| v.unwrap_or(0));
        println!("{:?}", regions);
        println!("{:?}", regions_unwrapped);
    }

    #[test]
    fn stencil() {
        use ndarray::prelude::*;
        let heights = arr2(&[
            [2, 1, 9, 9, 9, 4, 3, 2, 1, 0],
            [3, 9, 8, 7, 8, 9, 4, 9, 2, 1],
            [9, 8, 5, 6, 7, 8, 9, 8, 9, 2],
            [8, 7, 6, 7, 8, 9, 6, 7, 8, 9],
            [9, 8, 9, 9, 9, 6, 5, 6, 7, 8],
        ]);

        let boundary_value = 9;
        let mut regions = heights.mapv(|v| match v == boundary_value {
            true => None,
            false => Some(0),
        });
        println!("{:?}", regions);

        let mut counter = 1;

        for i in 0..regions.nrows() {
            for j in 0..regions.ncols() {
                let cell_value = regions.get((i, j)).unwrap();
                // Check if this is a boundary cell
                match cell_value {
                    None => (),
                    Some(_) => {
                        let view = regions
                            .slice(s![
                                i.saturating_sub(1)..regions.nrows().min(i + 2),
                                j.saturating_sub(1)..regions.ncols().min(j + 2),
                            ])
                            .into_owned();
                        let neighbour = view.iter().flatten().max();
                        match neighbour {
                            // all adjacents are None => boundary cells
                            None => {
                                counter += 1;
                                let cell = regions.get_mut((i, j)).unwrap();
                                *cell = Some(counter);
                                println!("GOT HERE!");
                            }
                            // one or more of the adjacent cells is not a boundary cell
                            Some(&region_index) => {
                                // if the max value here is 0, all non-boundary adjacent cells
                                // are untouched, so need a new region index value
                                if region_index == 0 {
                                    counter += 1;
                                    let cell = regions.get_mut((i, j)).unwrap();
                                    *cell = Some(counter);
                                // otherwise one of the adjacent cells belongs to a region
                                // which has already been given an index value. Copy it to
                                // this cell
                                } else {
                                    let cell = regions.get_mut((i, j)).unwrap();
                                    *cell = Some(region_index);
                                }
                            }
                        }
                        println!("({}, {})", i, j);
                        println!("view: {:?}", view);
                        println!(" = {:?}", view.get((i, j)));
                        // println!("adjacents: {:?}", adjacents.iter().flatten().collect::<Vec<_>>());
                        // println!("diagonals: {:?}", diagonals.iter().flatten().collect::<Vec<_>>());
                        println!("'neighbour' {:?}", neighbour);
                    }
                }
            }
        }
        println!("{}", heights);
        println!("{:?}", regions);
    }

    #[test]
    fn walk_around() {
        use ndarray::prelude::arr2;
        use crate::neighbours;

        let stencil = arr2(&[
            [Some(4), Some(3), Some(2)],
            [None,    Some(4), None   ],
            [Some(8), None,    Some(8)]
        ]);
        let centre = [1, 1];
        
        assert_eq!(
            neighbours(&stencil, &centre),
            vec![3,2,3,4]
        );
        println!("{:?}", neighbours(&stencil, &centre).iter().max());
    }

    #[test]
    fn laplace() {
        use ndarray::prelude::*;

        let heights = arr2(&[
            [2, 1, 9, 9, 9, 4, 3, 2, 1, 0],
            // [3, 9, 8, 7, 8, 9, 4, 9, 2, 1],
            // [9, 8, 5, 6, 7, 8, 9, 8, 9, 2],
            // [8, 7, 6, 7, 8, 9, 6, 7, 8, 9],
            // [9, 8, 9, 9, 9, 6, 5, 6, 7, 8],
        ]);

        for ((i, j), v) in heights.indexed_iter() {
            println!("({},{})", i, j);
            let mask = arr2(&[
                [-1.0, 1.0, -1.0]
                // [ 0., -1.,  0.],
                // [-1.,  4., -1.],
                // [ 0., -1.,  0.],
            ]);

            let h_i = mask.nrows() / 2; // kernel width, (or radius iff mask is square / symmetrical)
            let h_j = mask.ncols() / 2; // kernel width, (or radius iff mask is square / symmetrical)
            let ghost_below_i = h_i.saturating_sub(i); // effectively max(0, i - k_r)
            let ghost_below_j = h_j.saturating_sub(j); // "" for j
            let ghost_above_i = (i + h_i + 1).saturating_sub(heights.nrows()); // effectively max(0, i + k_r - nrows)
            let ghost_above_j = (j + h_j + 1).saturating_sub(heights.ncols()); // effectively max(0, i + k_r - ncols)

            let slice = s![
                i.saturating_sub(h_i + ghost_below_i)..=(i + h_i - ghost_above_i),
                j.saturating_sub(h_j + ghost_below_j)..=(j + h_j - ghost_above_j),
            ];

            let mask_slice = s![
                ghost_below_i..(mask.nrows() - ghost_above_i),
                ghost_below_j..(mask.ncols() - ghost_above_j),
            ];

            let view = heights.slice(slice).map(|v| *v as f64);
            let kernel = mask.slice(mask_slice);
            let gradient = (&kernel * &view).sum() / 2.0;

            println!(
                "{:?} in:\n{}\n * \n{}\n ↓ \n{}\n",
                v, view, kernel, gradient
            );
        }
    }
}

mod input {
    use ndarray::Array2;
    use std::fs::File;
    use std::io::{self, prelude::*};
    use std::path::Path;

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
