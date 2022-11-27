use regex::Regex;

#[derive(Default)]
struct Ingredient {
    // name: String,
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

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
    fn example_1() {
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
}
