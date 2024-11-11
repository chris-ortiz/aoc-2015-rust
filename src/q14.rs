use crate::q14::Status::RUNNING;
use itertools::Itertools;
use Status::RESTING;

pub fn q14() {
    let str = "Dancer can fly 27 km/s for 5 seconds, but then must rest for 132 seconds.
Cupid can fly 22 km/s for 2 seconds, but then must rest for 41 seconds.
Rudolph can fly 11 km/s for 5 seconds, but then must rest for 48 seconds.
Donner can fly 28 km/s for 5 seconds, but then must rest for 134 seconds.
Dasher can fly 4 km/s for 16 seconds, but then must rest for 55 seconds.
Blitzen can fly 14 km/s for 3 seconds, but then must rest for 38 seconds.
Prancer can fly 3 km/s for 21 seconds, but then must rest for 40 seconds.
Comet can fly 18 km/s for 6 seconds, but then must rest for 103 seconds.
Vixen can fly 18 km/s for 5 seconds, but then must rest for 84 seconds.";

    let input_time = 2503;

    let mut reindeers: Vec<Reindeer> = Vec::new();
    for line in str.lines() {
        let split: Vec<&str> = line.split(" ").into_iter().collect();
        let reindeer = Reindeer::new(usize::from_str_radix(split[3], 10).unwrap(),
                                     usize::from_str_radix(split[6], 10).unwrap(),
                                     usize::from_str_radix(split[13], 10).unwrap());
        reindeers.push(reindeer)
    }


    for i in 0..input_time {
        reindeers.iter_mut().for_each(|r| r.tick());
        let mut current_lead = reindeers.iter_mut()
            .max_set_by(|x, y| x.traveled.cmp(&y.traveled));
        current_lead.iter_mut().for_each(|mut r| r.points += 1)
    }

    println!("{:?}", reindeers.iter().max_by(|x, y| x.points.cmp(&y.points)).unwrap().points)
}

fn calc(total_time: usize, kmps: usize, endurance_time: usize, rest_time: usize) -> usize {
    let mut traveled = 0;
    let mut seconds_without_rest: usize = 0;
    let mut current_rest_time = 0;
    let mut status = RUNNING;

    for _ in 0..total_time {
        match status {
            RUNNING => {
                if seconds_without_rest < endurance_time {
                    traveled += kmps;
                    seconds_without_rest += 1
                } else {
                    status = RESTING;
                    seconds_without_rest = 0;
                    current_rest_time += 1
                }
            }
            RESTING => {
                if current_rest_time < rest_time {
                    current_rest_time += 1;
                } else {
                    status = RUNNING;
                    current_rest_time = 0;
                    seconds_without_rest += 1;
                    traveled += kmps;
                }
            }
        }
    }

    traveled
}

#[derive(Debug)]
struct Reindeer {
    status: Status,
    kmps: usize,
    endurance_time: usize,
    rest_time: usize,

    traveled: usize,
    seconds_without_rest: usize,
    current_rest_time: usize,
    points: usize,
}

impl Reindeer {
    fn new(kmps: usize, endurance_time: usize, rest_time: usize) -> Self {
        Self {
            status: RUNNING,
            kmps,
            endurance_time,
            rest_time,
            traveled: 0,
            seconds_without_rest: 0,
            current_rest_time: 0,
            points: 0,
        }
    }
    fn tick(&mut self) {
        match self.status {
            RUNNING => {
                if self.seconds_without_rest < self.endurance_time {
                    self.traveled += self.kmps;
                    self.seconds_without_rest += 1
                } else {
                    self.status = RESTING;
                    self.seconds_without_rest = 0;
                    self.current_rest_time += 1
                }
            }
            RESTING => {
                if self.current_rest_time < self.rest_time {
                    self.current_rest_time += 1;
                } else {
                    self.status = RUNNING;
                    self.current_rest_time = 0;
                    self.seconds_without_rest += 1;
                    self.traveled += self.kmps;
                }
            }
        }
    }
}

#[derive(Debug)]
enum Status {
    RESTING,
    RUNNING,
}


#[cfg(test)]
mod tests {
    use crate::q14::calc;

    #[test]
    fn test() {
        assert_eq!(1120, calc(1000, 14, 10, 127));
        assert_eq!(1056, calc(1000, 16, 11, 162));
    }
}