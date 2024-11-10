use indexmap::IndexMap;
use itertools::Itertools;
use std::collections::HashMap;

pub fn q9() {
    let input = String::from("AlphaCentauri to Snowdin = 66
AlphaCentauri to Tambi = 28
AlphaCentauri to Faerun = 60
AlphaCentauri to Norrath = 34
AlphaCentauri to Straylight = 34
AlphaCentauri to Tristram = 3
AlphaCentauri to Arbre = 108
Snowdin to Tambi = 22
Snowdin to Faerun = 12
Snowdin to Norrath = 91
Snowdin to Straylight = 121
Snowdin to Tristram = 111
Snowdin to Arbre = 71
Tambi to Faerun = 39
Tambi to Norrath = 113
Tambi to Straylight = 130
Tambi to Tristram = 35
Tambi to Arbre = 40
Faerun to Norrath = 63
Faerun to Straylight = 21
Faerun to Tristram = 57
Faerun to Arbre = 83
Norrath to Straylight = 9
Norrath to Tristram = 50
Norrath to Arbre = 60
Straylight to Tristram = 27
Straylight to Arbre = 81
Tristram to Arbre = 90");

    let map = parse_file(input);

    let mut res = Vec::new();
    let mut visited: IndexMap<String, usize> = IndexMap::new();
    calc(&mut res, &map, &mut visited);

    println!("{:?}", res.iter().max().unwrap());
}

fn calc(res: &mut Vec<usize>, map: &HashMap<String, City>, visited: &mut IndexMap<String, usize>) {
    if visited.len() == map.keys().len() {
        let mut dist_sum = 0;
        for (_, dist) in visited.clone() {
            dist_sum += dist
        }
        res.push(dist_sum);
    }

    for (name, city) in map {
        if !visited.contains_key(name) {
            if visited.len() == 0 {
                visited.insert(name.clone(), 0);
            } else {
                let (last_city_name, _) = visited.iter().last().unwrap();
                visited.insert(name.clone(), *city.neighbours.get(last_city_name).unwrap());
            }

            calc(res, map, visited);
            visited.pop();
        }
    }
}


fn parse_file(input: String) -> HashMap<String, City> {
    let mut map: HashMap<String, City> = HashMap::new();

    for line in input.split("\n") {
        let segments: Vec<&str> = line.split(" ")
            .map(|s| s.trim())
            .collect();

        let from_city_name = String::from(segments[0]);
        let to_city_name = String::from(segments[2]);
        let distance = segments[4].parse::<usize>().expect("a number that can be parsed");


        let to_city = map.entry(to_city_name.clone())
            .or_insert_with(|| City::new(to_city_name.clone()));

        to_city.neighbours.insert(from_city_name.clone(), distance);

        let from_city = map.entry(from_city_name.clone())
            .or_insert_with(|| City::new(from_city_name.clone()));


        from_city.neighbours.insert(to_city_name.clone(),
                                    distance,
        );
    }

    map
}

#[derive(Debug)]
struct City {
    name: String,
    neighbours: HashMap<String, usize>,
}

impl City {
    fn new(name: String) -> Self {
        City {
            name,
            neighbours: HashMap::new(),
        }
    }
}