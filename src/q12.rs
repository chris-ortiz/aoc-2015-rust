use fs::File;
use serde_json::{Map, Value};
use std::fs;

pub fn q12() {
    let json: Value = serde_json::from_reader(File::open("q12.json").unwrap()).unwrap();

    println!("sum: {}", visit(json))
}

fn visit(el: Value) -> i128 {
    match el {
        Value::Null => { 0 }
        Value::Bool(_) => { 0 }
        Value::Number(n) => { n.as_i128().unwrap() }
        Value::String(_) => { 0 }
        Value::Array(a) => {
            let mut sum = 0;
            for value in a {
                sum += visit(value)
            }
            sum
        }
        Value::Object(o) => {
            if is_red(&o) {
                return 0;
            }

            let mut sum = 0;
            for (_, value) in o {
                sum += visit(value)
            }
            sum
        }
    }
}

fn is_red(o: &Map<String, Value>) -> bool {
    for (_, value) in o {
        if value.is_string() && value.as_str().unwrap() == "red" {
            return true;
        }
    }
    false
}