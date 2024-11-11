use std::collections::HashMap;
use std::fs;
use std::path::Path;
use itertools::Itertools;
use crate::q13::HappinessType::{GAIN, LOSE};

pub fn q13() {
    let family = build_graph();

    let permutations = family.keys().permutations(family.len());
    let mut sums: Vec<_> = Vec::new();

    for perm in permutations {
        let mut sum: i32 = 0;

        for i in 0..8 {
            let person = family.get(perm[i]).unwrap();

            let neighbour = if i < 7 {
                perm[i + 1]
            } else {
                perm[0]
            };

            add(&mut sum, person.neighbours.get(neighbour).unwrap());
            add(&mut sum, family.get(neighbour).unwrap().neighbours.get(&person.name).unwrap());
        }
        sums.push(sum);
    }

    println!("{:?}", sums.iter().max());
}

fn add(sum: &mut i32, happiness: &Happiness) {
    match happiness.h_type {
        GAIN => { *sum += happiness.amount as i32 }
        LOSE => { *sum -= happiness.amount as i32 }
    }
}

fn build_graph() -> HashMap<String, Person> {
    let input = fs::read_to_string(Path::new("seating.txt"))
        .expect("Failed to read file");

    let mut family: HashMap<String, Person> = HashMap::new();

    for line in input.lines() {
        let split: Vec<&str> = line.split(" ").collect();
        let person_name = String::from(split[0]);

        let (neighbour_name, happiness) = get_happiness(&split);

        let mut person_option = family.get_mut(&person_name);

        if person_option.is_none() {
            let mut person = Person::new(person_name.clone());
            person.neighbours.insert(neighbour_name.clone(), happiness);
            family.insert(person_name.clone(), person);
        } else {
            let mut person = person_option.unwrap();
            person.neighbours.insert(neighbour_name.clone(), happiness);
        }
    }
    family
}

fn get_happiness(split: &Vec<&str>) -> (String, Happiness) {
    let h_type;
    if split[2].eq("gain") {
        h_type = GAIN
    } else {
        h_type = LOSE
    }

    let amount = u16::from_str_radix(split[3], 10).unwrap();

    (String::from(split[10].trim_end_matches(".")), Happiness {
        amount,
        h_type,
    })
}

#[derive(Debug)]
struct Person {
    name: String,
    neighbours: HashMap<String, Happiness>,
}

impl Person {
    fn new(name: String) -> Self {
        Self {
            name,
            neighbours: HashMap::new(),
        }
    }
}

#[derive(Debug)]
struct Happiness {
    amount: u16,
    h_type: HappinessType,
}

#[derive(Debug)]
enum HappinessType {
    GAIN,
    LOSE,
}