use crate::read_input::read_lines;

pub fn part_1() -> usize {
    calculate(false)
}

pub fn part_2() -> usize {
    calculate(true)
}

fn calculate(corners_are_on: bool) -> usize {
    let mut matrix: Matrix = read_lines(18).iter().map(parse_line).collect();

    for _ in 0..100 {
        matrix = tick(&matrix, corners_are_on);
    }

    matrix
        .iter()
        .map(|line| {
            line.iter()
                .filter(|value| **value)
                .collect::<Vec<_>>()
                .len()
        })
        .sum()
}

type Matrix = Vec<Vec<bool>>;

fn parse_line(line: &String) -> Vec<bool> {
    assert_eq!(line.len(), 100);

    line.chars().map(|char| char == '#').collect()
}

fn tick(matrix: &Matrix, corners_are_on: bool) -> Matrix {
    let offsets: [i32; 3] = [-1, 0, 1];

    matrix
        .iter()
        .enumerate()
        .map(|(x, line)| {
            line.iter()
                .enumerate()
                .map(|(y, value)| {
                    if corners_are_on {
                        match (x, y) {
                            (0, 0) | (0, 99) | (99, 0) | (99, 99) => return true,
                            _ => {}
                        };
                    }
                    let neighbours: u32 = offsets
                        .iter()
                        .map(|x_offset| {
                            offsets
                                .iter()
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
