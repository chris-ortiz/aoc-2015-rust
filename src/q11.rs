use itertools::Itertools;

// ascii 97 - 122 -> a - z
const MAX: u8 = 122;
const LOW: u8 = 97;
pub fn q11() {
    let mut input = String::from("cqjxxyzz");

    loop {
        let mut reversed_input: Vec<char> = input.chars().into_iter().rev().collect();
        increase_password(&mut reversed_input, 0, false);
        reversed_input.reverse();

        let res: String = reversed_input.iter().collect();
        println!("{:?}", res);

        if validate(&res) {
            break;
        }

        input = res
    }
}

fn validate(input: &String) -> bool {
    has_increasing_letters(input) && has_invalid_chars(input) && two_different_pairs(input)
}

fn two_different_pairs(input: &String) -> bool {
    let mut index: usize = 0;
    let mut input: Vec<char> = input.chars().into_iter().collect();
    let mut pair_count: usize = 0;
    let mut used_chars: Vec<char> = Vec::new();

    while index < input.len() - 1 {
        if input[index] == input[index + 1] && !used_chars.iter().contains(&input[index]) {
            pair_count += 1;
            used_chars.push(input[index])
        }

        if pair_count == 2 {
            return true;
        }

        index += 1
    }

    false
}

fn has_increasing_letters(input: &String) -> bool {
    let mut index: usize = 0;
    let mut input: Vec<char> = input.chars().into_iter().collect();

    while index < input.len() - 2 {
        if input[index] as u8 + 1 == input[index + 1] as u8
            && input[index + 2] as u8 == input[index + 1] as u8 + 1 {
            return true;
        }
        index += 1;
    }
    false
}

fn has_invalid_chars(input: &String) -> bool {
    !(input.contains("i") || input.contains("l") || input.contains("o"))
}

fn increase_password(input: &mut Vec<char>, start_index: usize, transfer: bool) {
    if start_index > input.len() - 1 {
        return;
    }

    if start_index == 0 || transfer {
        let (increased_char, transfer) = increase_char(input[start_index].clone());
        input[start_index] = increased_char;

        if transfer {
            increase_password(input, start_index + 1, transfer);
            if start_index + 1 >= input.len() {
                input.insert(0, 'a');
            }
        }
    }
}

fn increase_char(c: char) -> (char, bool) {
    let mut ascii_value = c as u8;
    let mut transfer = false;

    if ascii_value >= MAX {
        ascii_value = LOW;
        transfer = true
    } else {
        ascii_value += 1
    }

    (ascii_value as char, transfer)
}