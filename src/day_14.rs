struct Reindeer {
    flying_speed: u32,
    flying_time: u32,
    resting_time: u32,
}

#[derive(Debug)]
enum ReindeerState {
    HaveToFlyFor(u32),
    HaveToRestfor(u32),
}

struct ReindeerData {
    reindeer: Reindeer,
    state: ReindeerState,
    distance: u32,
    points: u32,
}

impl ReindeerData {
    fn new(reindeer: Reindeer) -> Self {
        let state = ReindeerState::HaveToFlyFor(reindeer.flying_time);

        Self {
            reindeer,
            state,
            distance: 0,
            points: 0,
        }
    }
    fn tick(&mut self) {
        use ReindeerState::*;

        match self.state {
            HaveToRestfor(ref mut time) => {
                *time -= 1;

                if *time == 0 {
                    self.state = HaveToFlyFor(self.reindeer.flying_time);
                }
            }
            HaveToFlyFor(ref mut time) => {
                self.distance += self.reindeer.flying_speed;
                *time -= 1;

                if *time == 0 {
                    self.state = HaveToRestfor(self.reindeer.resting_time);
                }
            }
        }
    }
}

pub fn part_1() -> u32 {
    let time = 2503;
    vec![comet_reindeer(), dancer_reindeer()]
        .iter()
        .map(|reindeer| distance(reindeer, time))
        .max()
        .unwrap()
}

pub fn part_2() -> u32 {
    let mut comet_data = ReindeerData::new(comet_reindeer());
    let mut dancer_data = ReindeerData::new(dancer_reindeer());

    for _ in 0..2503 {
        tick_reindeers(&mut comet_data, &mut dancer_data);
    }

    vec![comet_data.points, dancer_data.points]
        .iter()
        .max()
        .unwrap()
        .clone()
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

fn tick_reindeers(comet: &mut ReindeerData, dancer: &mut ReindeerData) {
    comet.tick();
    dancer.tick();

    if comet.distance > dancer.distance {
        comet.points += 1;
    } else if dancer.distance > comet.distance {
        dancer.points += 1;
    } else {
        comet.points += 1;
        dancer.points += 1;
    }
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

    #[test]
    fn part_2_competition_140_seconds() {
        let mut comet_data = ReindeerData::new(comet_reindeer());
        let mut dancer_data = ReindeerData::new(dancer_reindeer());

        for _ in 0..140 {
            tick_reindeers(&mut comet_data, &mut dancer_data);
        }

        assert_eq!(dancer_data.points, 139);
        assert_eq!(comet_data.points, 1);
    }

    #[test]
    fn part_2_competition_1000_seconds() {
        let mut comet_data = ReindeerData::new(comet_reindeer());
        let mut dancer_data = ReindeerData::new(dancer_reindeer());

        for _ in 0..1000 {
            tick_reindeers(&mut comet_data, &mut dancer_data);
        }

        assert_eq!(dancer_data.points, 689);
        assert_eq!(comet_data.points, 312);
    }
}
