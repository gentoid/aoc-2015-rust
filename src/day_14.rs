use itertools::Itertools;
use regex::Regex;

use crate::read_input::read_lines;

pub fn part_1() -> u32 {
    let reindeers = vec![comet_reindeer(), dancer_reindeer()];
    let the_winner = find_the_winner(reindeers, WinBy::Distance);

    the_winner.distance
}

pub fn part_2() -> u32 {
    let reindeers = read_lines(14).iter().map(parse_line).collect_vec();
    let the_winner = find_the_winner(reindeers, WinBy::Points);

    the_winner.points
}

enum WinBy {
    Distance,
    Points,
}

fn find_the_winner(reindeers: Vec<Reindeer>, win_by: WinBy) -> ReindeerData {
    let mut reindeers = reindeers
        .into_iter()
        .map(ReindeerData::new)
        .collect::<Vec<_>>();

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

fn parse_line(line: &String) -> Reindeer {
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
    fn part_2_competition_140_seconds() {
        let comet_data = ReindeerData::new(comet_reindeer());
        let dancer_data = ReindeerData::new(dancer_reindeer());

        let mut reindeers = vec![comet_data, dancer_data];

        for _ in 0..140 {
            reindeers = tick_reindeers(reindeers);
        }

        assert_eq!(reindeers.get(0).unwrap().points, 1);
        assert_eq!(reindeers.get(1).unwrap().points, 139);
    }

    #[test]
    fn part_2_competition_1000_seconds() {
        let comet_data = ReindeerData::new(comet_reindeer());
        let dancer_data = ReindeerData::new(dancer_reindeer());

        let mut reindeers = vec![comet_data, dancer_data];

        for _ in 0..1000 {
            reindeers = tick_reindeers(reindeers);
        }

        assert_eq!(reindeers.get(0).unwrap().points, 312);
        assert_eq!(reindeers.get(1).unwrap().points, 689);
    }
}
