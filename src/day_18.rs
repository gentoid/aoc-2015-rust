use rayon::prelude::*;

type Matrix = Vec<Vec<bool>>;

fn parse_line(line: &String) -> Vec<bool> {
    assert_eq!(line.len(), 100);

    line.par_chars().map(|char| char == '#').collect()
}

fn tick(matrix: &Matrix) -> Matrix {
    let offsets: [i32; 3] = [-1, 0, 1];

    matrix
        .par_iter()
        .enumerate()
        .map(|(x, line)| {
            line.par_iter()
                .enumerate()
                .map(|(y, value)| {
                    let neighbours: u32 = offsets
                        .par_iter()
                        .map(|x_offset| {
                            offsets
                                .par_iter()
                                .map(|y_offset| {
                                    if *x_offset == 0 && *y_offset == 0 {
                                        return 0;
                                    }

                                    if (x as i32 + x_offset) < 0 {
                                        return 0;
                                    }

                                    if (y as i32 + y_offset) < 0 {
                                        return 0;
                                    }

                                    match matrix.get((x as i32 + x_offset) as usize) {
                                        None => 0,
                                        Some(offset_line) => {
                                            match offset_line.get((y as i32 + y_offset) as usize) {
                                                None => 0,
                                                Some(offset_value) => {
                                                    if *offset_value {
                                                        1
                                                    } else {
                                                        0
                                                    }
                                                }
                                            }
                                        }
                                    }
                                })
                                .sum::<u32>()
                        })
                        .sum();

                    neighbours == 3 || (*value && neighbours == 2)
                })
                .collect()
        })
        .collect()
}
