use std::fs;
use std::path::Path;

pub fn q8() {
    let input = fs::read_to_string(Path::new("matchsticks.txt"))
        .expect("Failed to read file");

    let tokens = tokenize(input);

    println!("{:?}", tokens);

    let size = calculate_diff(tokens);

    println!("size {}", size)
}

fn calculate_diff(tokens: Vec<Token>) -> usize {
    let mut size = 0;

    for token in tokens {
        size += token.literal_size - token.in_memory_size;
    }

    size
}

const ILLEGAL_CHARS: [char; 2] = ['\\', '"'];
const START_OF_STRING: char = '"';

fn tokenize(input: String) -> Vec<Token> {
    println!("tokenizing: {}", input);

    let mut tokens: Vec<Token> = Vec::new();
    let chars: Vec<char> = input.chars().collect();
    let mut i: usize = 0;

    while i < chars.len() {
        if chars[i] == START_OF_STRING {
            let token = read_string(&chars, i);
            i += token.literal_size;

            tokens.push(token);
        } else {
            i += 1;
        }
    }

    tokens
}

fn read_string(chars: &Vec<char>, start_index: usize) -> Token {
    let mut i = start_index;
    let mut str: Vec<char> = Vec::new();

    while i < chars.len() && chars[i] != '\n' {
        let is_illegal_char = ILLEGAL_CHARS.contains(&chars[i]);

        if !is_illegal_char {
            str.push(chars[i])
        } else if chars[i] == '\\' && i + 3 < chars.len() && chars[i + 1] == 'x' {
            // let h1 = chars[i + 2];
            // let h2 = chars[i + 3];
            //
            // let hex_string = format!("{}{}", h1, h2);
            // let hex_string_str = hex_string.as_str();
            // let result = u32::from_str_radix(hex_string_str, 16).unwrap();
            // let char = char::from_u32(result).unwrap();

            str.push('?');

            i += 3;
        } else if chars[i] == '\\' && i + 1 < chars.len() && chars[i + 1] == '\\' {
            str.push('\\');
            i += 1
        } else if chars[i] == '\\' && i + 1 < chars.len() && chars[i + 1] == '"' {
            str.push('"');
            i += 1
        }


        i += 1;
    }

    let clean_value: String = str.iter().collect();
    let in_memory_size = clean_value.len();

    Token {
        string_value: clean_value,
        string_literal: chars[start_index..i].iter().collect(),
        in_memory_size,
        literal_size: i - start_index,
        abs_index: start_index,
    }
}


#[derive(Debug)]
struct Token {
    string_value: String,
    string_literal: String,
    literal_size: usize,
    abs_index: usize,
    in_memory_size: usize,
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex() {
        let token = tokenize(String::from("\"\\x27\""));
        assert_eq!("\"\\x27\"", token[0].string_literal);
        assert_eq!("?", token[0].string_value);
        assert_eq!(6, token[0].literal_size);
        assert_eq!(1, token[0].in_memory_size);
    }

    #[test]
    fn test_tokenizer() {
        let token = tokenize(String::from("\"\""));
        assert_eq!("", token[0].string_value);
        assert_eq!(2, token[0].literal_size);
        assert_eq!(0, token[0].in_memory_size);

        let token = tokenize(String::from("\"aaa\\\"aaa\""));
        assert_eq!(10, token[0].literal_size);
        assert_eq!(7, token[0].in_memory_size);
    }

    #[test]
    fn double_backslash() {
        let token = tokenize(String::from("\"\\\\\""));
        println!("{:?}", token);
        assert_eq!(4, token[0].literal_size);
        assert_eq!(1, token[0].in_memory_size);
    }

    #[test]
    fn double_backslash_with_x() {
        let token = tokenize(String::from("\"lhyjky\\\\m\\\"pvnm\\\\xmynpxnlhndmahjl\""));
        println!("{:?}", token);
        assert_eq!(35, token[0].literal_size);
        assert_eq!(30, token[0].in_memory_size);
    }

    #[test]
    fn sample_1() {
        let token = tokenize(String::from("\"sjdivfriyaaqa\\xd2v\\\"k\\\"mpcu\\\"yyu\\\"en\""));
        println!("{:?}", token);
        assert_eq!("sjdivfriyaaqa?v\"k\"mpcu\"yyu\"en", token[0].string_value);
    }

    #[test]
    fn sample_2() {
        let token = tokenize(String::from("\"xziq\\\\\\x18ybyv\\x9am\\\"neacoqjzytertisysza\""));
        println!("{:?}", token);
        assert_eq!(32, token[0].in_memory_size);
    }

    #[test]
    fn test_calculation() {
        let token = tokenize(String::from("\"\"\n\"abc\"\n\"aaa\\\"aaa\"\n\"\\x27\""));
        println!("{:?}", token);
        let diff = calculate_diff(token);
        assert_eq!(12, diff);
    }
}
