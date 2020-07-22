use std::collections::HashMap;

struct LightsField {
    matrix: HashMap<(u32, u32), bool>,
}

impl LightsField {
    fn new() -> Self {
        LightsField {
            matrix: HashMap::new()
        }
    }

    fn turn_on(&mut self, from: (u32, u32), to: (u32, u32)) {
        for x in from.0..=to.0 {
            for y in from.1..=to.1 {
                let light = self.matrix.entry((x, y)).or_insert(true);
                *light = true;
            }
        }
    }

    fn turn_off(&mut self, from: (u32, u32), to: (u32, u32)) {
        for x in from.0..=to.0 {
            for y in from.1..=to.1 {
                let light = self.matrix.entry((x, y)).or_insert(false);
                *light = false;
            }
        }
    }

    fn toggle(&mut self, from: (u32, u32), to: (u32, u32)) {
        for x in from.0..=to.0 {
            for y in from.1..=to.1 {
                let light = self.matrix.entry((x, y)).or_insert(false);
                *light = !*light;
            }
        }
    }

    fn count(&self) -> usize {
        self.matrix.iter().filter(|(_, &value)| value).collect::<Vec<_>>().len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_lights_after_init() {
        assert_eq!(LightsField::new().count(), 0)
    }

    #[test]
    fn turns_on_small_square() {
        let mut lights = LightsField::new();
        lights.turn_on((1, 1), (4, 4));
        assert_eq!(lights.count(), 16);
    }
}