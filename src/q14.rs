use itertools::Itertools;
use Status::RESTING;
use crate::q14::Status::RUNNING;

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
    let mut times: Vec<_> = Vec::new();

    for line in str.lines() {
        let split: Vec<&str> = line.split(" ").into_iter().collect();
        let time = calc(input_time, usize::from_str_radix(split[3], 10).unwrap(),
                        usize::from_str_radix(split[6], 10).unwrap(),
                        usize::from_str_radix(split[13], 10).unwrap());
        println!("{}", time);
        times.push(time)
    }

    print!("max {:?}", times.iter().max())
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