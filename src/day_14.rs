struct Reindeer {
    flying_speed: u32,
    flying_time: u32,
    resting_time: u32,
}

pub fn part_1() -> u32 {
    let time = 2503;
    vec![comet_reindeer(), dancer_reindeer()]
        .iter()
        .map(|reindeer| distance(reindeer, time))
        .max()
        .unwrap()
}

fn comet_reindeer() -> Reindeer {
    Reindeer {
        flying_speed: 14,
        flying_time: 10,
        resting_time: 127,
    }
}

fn dancer_reindeer() -> Reindeer {
    Reindeer {
        flying_speed: 16,
        flying_time: 11,
        resting_time: 162,
    }
}

fn distance(reindeer: &Reindeer, time: u32) -> u32 {
    let full_cicle = reindeer.flying_time + reindeer.resting_time;
    let km_per_cicle = reindeer.flying_time * reindeer.flying_speed;

    if time <= reindeer.flying_time {
        return time * reindeer.flying_speed;
    }

    if time < full_cicle {
        return km_per_cicle;
    }

    let cicles = time / full_cicle;

    cicles * km_per_cicle + distance(reindeer, time - cicles * full_cicle)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn comet_reindeer_distances() {
        let comet = comet_reindeer();

        assert_eq!(distance(&comet, 1), 14);
        assert_eq!(distance(&comet, 10), 140);
        assert_eq!(distance(&comet, 1000), 1120);
    }

    #[test]
    fn dancer_reindeer_distances() {
        let dancer = dancer_reindeer();

        assert_eq!(distance(&dancer, 1), 16);
        assert_eq!(distance(&dancer, 10), 160);
        assert_eq!(distance(&dancer, 1000), 1056);
    }
}
