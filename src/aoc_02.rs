use crate::read_input::read_lines;

fn input() -> Vec<[u32; 3]> {
    let mut dimensions = vec![];
    for line in read_lines(2) {
        let split: Vec<u32> = line.split("x").map(|dim| dim.parse::<u32>().unwrap()).take(3).collect();
        dimensions.push([split[0], split[1], split[2]]);
    }

    dimensions
}

pub fn aoc_02_01() -> u32 {
    let mut area = 0;

    for dim in input() {
        area += calculate_area(dim[0], dim[1], dim[2]);
    }

    area
}

fn calculate_area(x: u32, y: u32, z: u32) -> u32 {
    let (min_a, min_b) = find_minimal(x, y, z);
    2 * (x * y + y * z + x * z) + min_a * min_b
}

fn find_minimal(x: u32, y: u32, z: u32) -> (u32, u32) {
    let mut min_a = x;
    let mut min_b = y;

    if x > z {
        min_a = z;
        if y > x {
            min_b = x;
        }
    } else if y > z {
        min_b = z;
        if x > y {
            min_a = y;
        }
    }

    (min_a, min_b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pack_01() {
        assert_eq!(58, calculate_area(2, 3, 4));
    }

    #[test]
    fn pack_02() {
        assert_eq!(43, calculate_area(1, 1, 10));
    }

    #[test]
    fn find_min_01() {
        assert_eq!((1, 2), find_minimal(1, 2, 3));
    }

    #[test]
    fn find_min_02() {
        assert_eq!((1, 2), find_minimal(3, 2, 1));
    }

    #[test]
    fn find_min_03() {
        assert_eq!((1, 2), find_minimal(2, 3, 1));
    }
}
