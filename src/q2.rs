use std::cmp::min;
use std::fs;
use std::path::Path;

pub fn q2() {
    let data = fs::read_to_string(Path::new("present_sizes.txt"))
        .expect("Failed to read file");

    let mut sum_paper = 0;
    let mut sum_ribbon = 0;

    for line in data.lines() {
        let present = parse(line);
        sum_paper += present.paper();
        sum_ribbon += present.ribbon();
    }

    println!("paper: {sum_paper:?}\nribbon: {sum_ribbon:?}");
}

#[derive(Debug)]
struct Present {
    l: i32,
    w: i32,
    h: i32,
}

impl Present {
    fn paper(&self) -> i32 {
        let side1 = self.l * self.w;
        let side2 = self.w * self.h;
        let side3 = self.h * self.l;

        let sum = (2 * side1) + (2 * side2) + (2 * side3);

        let smallest_side = min(side1, min(side2, side3));

        sum + smallest_side
    }
    fn ribbon(&self) -> i32 {
        let volume = self.w * self.h * self.l;

        let perimeter1 = 2 * (self.h + self.w);
        let perimeter2 = 2 * (self.w + self.l);
        let perimeter3 = 2 * (self.h + self.l);


        volume + min(perimeter1, min(perimeter2, perimeter3))
    }
}

fn parse(line: &str) -> Present {
    let fragments: Vec<&str> = line.split('x').collect();
    let l = fragments[0].parse::<i32>().unwrap();
    let w = fragments[1].parse::<i32>().unwrap();
    let h = fragments[2].parse::<i32>().unwrap();

    Present {
        l,
        w,
        h,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let p = parse("2x3x4");
        assert_eq!(34, p.ribbon());

        let p = parse("1x1x10");
        assert_eq!(14, p.ribbon())
    }
}