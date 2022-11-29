use rayon::prelude::*;

use crate::read_input::read_lines;

pub fn part_1() -> usize {
    let mut matrix: Matrix = read_lines(18).par_iter().map(parse_line).collect();

    for _ in 0..100 {
        matrix = tick(&matrix);
    }

    matrix
        .par_iter()
        .map(|line| {
            line.par_iter()
                .filter(|value| **value)
                .collect::<Vec<_>>()
                .len()
        })
        .sum()
}

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
