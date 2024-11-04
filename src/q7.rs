use std::collections::HashMap;
use std::cell::RefCell;
use std::fs;
use std::path::Path;

#[derive(Debug)]
struct Wire {
    identifier: String,
    input: String,
    signal: Option<u16>,
}

impl Wire {
    fn new(identifier: String, input: String) -> Self {
        Self {
            identifier,
            input,
            signal: None,
        }
    }

    fn compute_signal(&mut self, wires: &HashMap<&str, RefCell<Wire>>) -> u16 {
        if self.signal.is_none() {
            let tokens: Vec<&str> = self.input.split_whitespace().collect();

            if tokens.len() == 1 {
                let current = tokens[0];
                // token is a value
                self.signal = Wire::resolve_value(wires, current);
            } else if tokens.len() == 2 {
                // NOT
                self.signal = Some(!Wire::resolve_value(wires, tokens[1]).unwrap());
            } else if tokens.len() == 3 {
                self.signal = match tokens.as_slice() {
                    [var1, "AND", var2] => Some(Wire::resolve_value(wires, var1).unwrap() & Wire::resolve_value(wires, var2).unwrap()),
                    [var1, "LSHIFT", var2] => Some(Wire::resolve_value(wires, var1).unwrap() << Wire::resolve_value(wires, var2).unwrap()),
                    [var1, "RSHIFT", var2] => Some(Wire::resolve_value(wires, var1).unwrap() >> Wire::resolve_value(wires, var2).unwrap()),
                    [var1, "OR", var2] => Some(Wire::resolve_value(wires, var1).unwrap() | Wire::resolve_value(wires, var2).unwrap()),
                    _ => panic!("unknown command")
                }
            }
        }

        self.signal.unwrap()
    }

    fn resolve_value(wires: &HashMap<&str, RefCell<Wire>>, reference_or_value: &str) -> Option<u16> {
        if reference_or_value.chars().next().unwrap().is_numeric() {
            Some(reference_or_value.parse().unwrap())
        }
        // token is a reference to another wire
        else {
            let mut wire = wires.get(reference_or_value).unwrap().borrow_mut();
            Some(wire.compute_signal(wires))
        }
    }
}

pub fn q7() {
    let input = fs::read_to_string(Path::new("wire-input.txt"))
        .expect("Failed to read file");
    let lines = input.lines();

    let mut wires: HashMap<&str, RefCell<Wire>> = HashMap::new();

    for line in lines {
        let segments: Vec<&str> = line.split("->").collect();
        let wire_identifier = segments[1].trim();

        let wire = Wire::new(String::from(wire_identifier), String::from(segments[0].trim()));
        wires.insert(wire_identifier, RefCell::new(wire));
    }

    let keys: Vec<_> = wires.keys().cloned().collect();

    // compute wires
    for key in keys {
        let wire = wires.get(key).unwrap();
        let mut wire_borrow = wire.borrow_mut();
        let _ = wire_borrow.compute_signal(&wires);
    }

    println!("wire a has signal {:?}", wires.get("a"))
}
