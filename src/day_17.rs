use itertools::Itertools;

use crate::read_input::read_lines;

pub fn part_1() -> usize {
    let containers = read_lines(17)
        .iter()
        .map(|line| line.parse::<u32>().unwrap())
        .collect_vec();

    combinations(150, &containers).len()
}

fn combinations(liters: u32, containers: &[u32]) -> Vec<Vec<u32>> {
    match containers {
        [] => vec![],
        [head] => {
            if *head == liters {
                vec![vec![*head]]
            } else {
                vec![]
            }
        }
        [head, tail @ ..] => {
            let mut output = vec![];

            output.extend(combinations(liters, &[*head]));
            output.extend(combinations(liters, tail));

            if *head < liters {
                for combination in combinations(liters - head, tail) {
                    let mut tmp = vec![*head];
                    tmp.extend(combination);
                    output.extend(vec![tmp]);
                }
            }

            output
        }
    }
}
