use regex::Regex;

#[derive(Clone, Debug, Default)]
struct Ingredient {
    // name: String,
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

#[derive(Clone, Debug)]
struct Teaspoon {
    quantity: u32,
    ingredient: Ingredient,
}

fn parse_line(line: &str) -> Ingredient {
    let temlate = Regex::new(
        r"(\w+): capacity (-?\d+) durability (-?\d+), flavor (-?\d+), texture (-?\d+), calories (-?\d+)",
    )
    .unwrap();
    let captures = temlate.captures(line).unwrap();

    Ingredient {
        // name: captures[1].to_owned(),
        capacity: captures[2].parse::<i32>().unwrap(),
        durability: captures[3].parse::<i32>().unwrap(),
        flavor: captures[4].parse::<i32>().unwrap(),
        texture: captures[5].parse::<i32>().unwrap(),
        calories: captures[6].parse::<i32>().unwrap(),
    }
}

fn calculate_score(teaspoons: &Vec<Teaspoon>) -> u32 {
    let mut total = Ingredient {
        // name: String::from("total"),
        ..Default::default()
    };

    for teaspoon in teaspoons {
        total.capacity += (teaspoon.quantity as i32) * teaspoon.ingredient.capacity;
        total.durability += (teaspoon.quantity as i32) * teaspoon.ingredient.durability;
        total.flavor += (teaspoon.quantity as i32) * teaspoon.ingredient.flavor;
        total.texture += (teaspoon.quantity as i32) * teaspoon.ingredient.texture;
    }

    if total.capacity < 0 || total.durability < 0 || total.flavor < 0 || total.texture < 0 {
        return 0;
    }

    let output = total.capacity * total.durability * total.flavor * total.texture;
    assert!(output >= 0);

    output as u32
}

fn combinations(quantity_left: u32, ingredients: &[Ingredient]) -> Vec<Vec<Teaspoon>> {
    match ingredients {
        [] => vec![],
        [head] => vec![vec![Teaspoon {
            quantity: quantity_left,
            ingredient: (*head).clone(),
        }]],
        [head, tail @ ..] => {
            let mut output = vec![];

            for quantity in 0..=quantity_left {
                let teaspoon= Teaspoon {
                    quantity,
                    ingredient: (*head).clone(),
                };

                for inner in combinations(quantity_left - quantity, tail) {
                    let mut tmp = vec![teaspoon.clone()];
                    tmp.extend(inner);
                    output.push(tmp);
                }
        
            }

            output
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_BUTTERSOTCH: Ingredient = Ingredient {
        // name: String::from("Buttersotch"),
        capacity: -1,
        durability: -2,
        flavor: 6,
        texture: 3,
        calories: 8,
    };

    const EXAMPLE_CINNAMON: Ingredient = Ingredient {
        // name: String::from("Cinnamon"),
        capacity: 2,
        durability: 3,
        flavor: -2,
        texture: -1,
        calories: 3,
    };

    #[test]
    fn calculates_score() {
        assert_eq!(
            calculate_score(&vec![
                Teaspoon {
                    quantity: 44,
                    ingredient: EXAMPLE_BUTTERSOTCH
                },
                Teaspoon {
                    quantity: 56,
                    ingredient: EXAMPLE_CINNAMON
                }
            ]),
            62842880
        );
    }

    #[test]
    fn finds_all_combinations() {
        let ingredients = vec![EXAMPLE_BUTTERSOTCH, EXAMPLE_BUTTERSOTCH, EXAMPLE_CINNAMON];

        assert_eq!(combinations(1, &ingredients).len(), 3);
        assert_eq!(combinations(2, &ingredients).len(), 6);
        assert_eq!(combinations(3, &ingredients).len(), 10);
        assert_eq!(combinations(4, &ingredients).len(), 15);
        assert_eq!(combinations(5, &ingredients).len(), 21);
    }
}
