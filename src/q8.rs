use std::fs;
use std::path::Path;

pub fn q8() {
    let input = fs::read_to_string(Path::new("matchsticks.txt"))
        .expect("Failed to read file");

    let tokens = tokenize(input);

    println!("{:?}", tokens);

    let size = calculate_diff(&tokens);

    println!("size {}", size);

    let size = calculate_diff_encoded(tokens);

    println!("encoded size {}", size)
}

fn calculate_diff(tokens: &Vec<Token>) -> usize {
    let mut size = 0;

    for token in tokens {
        size += token.literal_size - token.in_memory_size;
    }

    size
}

fn calculate_diff_encoded(tokens: Vec<Token>) -> usize {
    let mut size = 0;

    for token in tokens {
        size += token.new_string_size - token.literal_size;
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
    let (i, escaped_string) = escape(chars, start_index);
    let encoded_string = encode(chars, start_index);


    let in_memory_size = escaped_string.len();
    let encoded_size = encoded_string.len();

    Token {
        string_value: escaped_string,
        string_literal: chars[start_index..i].iter().collect(),
        in_memory_size,
        literal_size: i - start_index,
        abs_index: start_index,
        new_string: encoded_string,
        new_string_size: encoded_size,
    }
}

fn encode(chars: &Vec<char>, start_index: usize) -> String {
    let mut i = start_index;
    let mut str: Vec<char> = Vec::new();

    str.push('"');
    while i < chars.len() && chars[i] != '\n' {
        if chars[i] == '"' {
            str.push('\\');
            str.push('"');
        } else if chars[i] == '\\' {
            str.push('\\');
            str.push('\\');
        } else {
            str.push(chars[i]);
        }

        i += 1
    }

    str.push('"');

    str.iter().collect()
}

fn escape(chars: &Vec<char>, start_index: usize) -> (usize, String) {
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

    (i, str.iter().collect())
}

#[derive(Debug)]
struct Token {
    abs_index: usize,
    string_value: String,
    string_literal: String,
    new_string: String,
    literal_size: usize,
    in_memory_size: usize,
    new_string_size: usize,
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
        let diff = calculate_diff(&token);
        assert_eq!(12, diff);
    }

    #[test]
    fn test_encoding() {
        let token = tokenize(String::from("\"\""));
        assert_eq!("\"\\\"\\\"\"", token[0].new_string);
    }
}
