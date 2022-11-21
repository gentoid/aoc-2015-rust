use regex::Regex;

use crate::read_input::read_lines;

pub fn part_1() -> u32 {
    let the_winner = find_the_winner(
        vec![
            ReindeerData::new(comet_reindeer()),
            ReindeerData::new(dancer_reindeer()),
        ],
        WinBy::Distance,
    );

    the_winner.distance
}

pub fn part_2() -> u32 {
    let reindeers = read_lines(14)
        .iter()
        .map(|line| ReindeerData::new(parse_line(line)))
        .collect::<Vec<_>>();

    let the_winner = find_the_winner(reindeers, WinBy::Points);

    the_winner.points
}

enum WinBy {
    Distance,
    Points,
}

fn find_the_winner(mut reindeers: Vec<ReindeerData>, win_by: WinBy) -> ReindeerData {
    for _ in 0..2503 {
        reindeers = tick_reindeers(reindeers);
    }

    let mut output: ReindeerData = reindeers.pop().unwrap();

    match win_by {
        WinBy::Distance => {
            for reindeer in reindeers {
                if reindeer.distance > output.distance {
                    output = reindeer;
                }
            }
        }
        WinBy::Points => {
            for reindeer in reindeers {
                if reindeer.points > output.points {
                    output = reindeer;
                }
            }
        }
    }
    output
}

fn parse_line(line: &str) -> Reindeer {
    let temlate = Regex::new(
        r"(\w+) can fly (\d+) km/s for (\d+) seconds, but then must rest for (\d+) seconds.",
    )
    .unwrap();
    let captures = temlate.captures(line).unwrap();

    Reindeer {
        flying_speed: captures[2].parse::<u32>().unwrap(),
        flying_time: captures[3].parse::<u32>().unwrap(),
        resting_time: captures[4].parse::<u32>().unwrap(),
    }
}

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

fn tick_reindeers(mut reindeers: Vec<ReindeerData>) -> Vec<ReindeerData> {
    for reindeer_data in &mut reindeers {
        reindeer_data.tick();
    }

    let max_distance = reindeers
        .iter()
        .map(|reindeer_data| reindeer_data.distance)
        .max()
        .unwrap();

    for reindeer_data in &mut reindeers {
        if reindeer_data.distance == max_distance {
            reindeer_data.points += 1;
        }
    }

    reindeers
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

        let mut reindeers = vec![comet_data, dancer_data];

        for _ in 0..140 {
            reindeers = tick_reindeers(reindeers);
        }

        assert_eq!(reindeers.get(0).unwrap().points, 1);
        assert_eq!(reindeers.get(1).unwrap().points, 139);
    }

    #[test]
    fn part_2_competition_1000_seconds() {
        let mut comet_data = ReindeerData::new(comet_reindeer());
        let mut dancer_data = ReindeerData::new(dancer_reindeer());

        let mut reindeers = vec![comet_data, dancer_data];

        for _ in 0..1000 {
            reindeers = tick_reindeers(reindeers);
        }

        assert_eq!(reindeers.get(0).unwrap().points, 312);
        assert_eq!(reindeers.get(1).unwrap().points, 689);
    }
}
