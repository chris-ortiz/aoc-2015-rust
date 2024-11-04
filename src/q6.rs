use crate::q6::CommandType::{OFF, ON, TOGGLE};
use std::fs;
use std::hash::Hash;
use std::path::Path;

#[derive(Debug)]
struct Command {
    start: Pos,
    end: Pos,
    command_type: CommandType,
}

#[derive(Debug)]
struct Pos {
    x: usize,
    y: usize,
}

#[derive(Debug)]
enum CommandType {
    ON,
    OFF,
    TOGGLE,
}

#[derive(Copy, Clone)]
struct LightBulb {
    brightness: u32,
}

impl LightBulb {
    fn new() -> Self {
        Self {
            brightness: 0
        }
    }
    fn on(&mut self) {
        self.brightness += 1
    }
    fn off(&mut self) {
        if self.brightness == 0 {
            return;
        }

        self.brightness -= 1
    }
    fn toggle(&mut self) {
        self.brightness += 2
    }
}

impl Command {
    fn exec(&self, input: &mut LightBulb) {
        match self.command_type {
            ON => input.on(),
            OFF => input.off(),
            TOGGLE => input.toggle(),
        }
    }
}

impl CommandType {
    fn value(&self) -> &str {
        match *self {
            ON => "turn on",
            OFF => "turn off",
            TOGGLE => "toggle",
        }
    }
}

fn parse(command_type: CommandType, line: &str) -> Command {
    let mut split = line.trim_start_matches(command_type.value())
        .split("through");
    let start_uints = extract_uints(split.next().unwrap());
    let end_uints = extract_uints(split.next().unwrap());

    Command {
        start: Pos {
            x: *start_uints.first().unwrap(),
            y: *start_uints.last().unwrap(),
        },
        end: Pos {
            x: *end_uints.first().unwrap(),
            y: *end_uints.last().unwrap(),
        },
        command_type,
    }
}

fn extract_uints(s: &str) -> Vec<usize> {
    s.trim()
        .split(",")
        .map(|s| s.parse::<usize>().expect("Failed to parse value"))
        .collect()
}

pub fn q6() {
    let mut grid: [[LightBulb; 1000]; 1000] = [[LightBulb { brightness: 0 }; 1000]; 1000];

    let input = fs::read_to_string(Path::new("light_commands.txt"))
        .expect("Failed to read file");
    let lines = input.lines();


    for line in lines {
        let command = if line.starts_with("turn on") {
            parse(ON, line)
        } else if line.starts_with("turn off") {
            parse(OFF, line)
        } else if line.starts_with("toggle") {
            parse(TOGGLE, line)
        } else {
            panic!("{}", format!("unknown signal {:?}", line))
        };

        execute_command(&mut grid, command);
    }


    println!("brightness: {}", brightness(&mut grid))
}

fn execute_command(mut grid: &mut [[LightBulb; 1000]; 1000], command: Command) {
    for i in command.start.x..=command.end.x {
        for j in command.start.y..=command.end.y {
            command.exec(&mut grid[i][j])
        }
    }
}

fn brightness(grid: &[[LightBulb; 1000]; 1000]) -> u32 {
    let mut brightness: u32 = 0;
    for x in 0..1000 {
        for y in 0..1000 {
            brightness += grid[x][y].brightness
        }
    }
    brightness
}