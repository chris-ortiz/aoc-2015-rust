use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

// b
pub fn q5() {
    let input = fs::read_to_string(Path::new("good_words.txt"))
        .expect("Failed to read file");
    let words = input.lines();

    let mut word_count = 0;

    for word in words {
        if is_very_nice_word(word) {
            word_count += 1
        }
    }

    println!("very nice strings {:?}", word_count);
}

fn is_very_nice_word(word: &str) -> bool {
    let mut has_repeating_letters = false;
    let characters: Vec<char> = word.chars().collect();

    let mut has_repeating_segments = false;
    let mut segments: HashMap<String, usize> = HashMap::new();

    for i in 0..characters.len() {
        if !has_repeating_letters && i as i32 - 2 >= 0 && characters[i - 2] == characters[i] {
            has_repeating_letters = true;
        }

        if !has_repeating_segments && i > 0 {
            let segment = format!("{}{}", characters[i - 1], characters[i]);
            let last_index_of_segment = segments.get(&segment);

            if last_index_of_segment.is_some() {
                if !last_index_of_segment.unwrap().eq(&(i - 1)) {
                    has_repeating_segments = true;
                    segments.insert(segment, i);
                }
            } else {
                segments.insert(segment, i);
            }
        }

        if has_repeating_letters && has_repeating_segments {
            println!("Nice word: {}", word);
            return true;
        }
    }

    false
}

pub fn q5a() {
    const MIN_VOWELS: i32 = 3;
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let disallowed_strings = ["ab", "cd", "pq", "xy"];

    let input = fs::read_to_string(Path::new("good_words.txt"))
        .expect("Failed to read file");

    let mut nice_words_count = 0;
    let words = input.lines();


    for word in words {
        let mut last_char: Option<char> = None;

        // check for disallowed strings
        let mut has_disallowed_string = false;
        for ds in disallowed_strings {
            if word.contains(&ds) {
                has_disallowed_string = true;
                break;
            }
        }

        if has_disallowed_string {
            continue;
        }


        // iterate through characters checking for
        // good word indicators

        let mut vowel_count = 0;
        let mut has_3_vowels = false;
        let mut has_chars_twice_in_a_row = false;
        for c in word.chars() {
            if vowels.contains(&c) {
                if vowel_count == 2 {
                    has_3_vowels = true;
                } else {
                    vowel_count += 1;
                }
            }

            if last_char.is_some_and(|lc| lc == c) {
                has_chars_twice_in_a_row = true;
            }

            if has_3_vowels && has_chars_twice_in_a_row {
                nice_words_count += 1;
                break;
            }

            last_char = Some(c);
        }
    }

    println!("nice strings {:?}", nice_words_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(true, is_very_nice_word("qjhvhtzxzqqjkmpb"));
        assert_eq!(true, is_very_nice_word("xxyxx"));
        assert_eq!(false, is_very_nice_word("aaa"));
        assert_eq!(true, is_very_nice_word("aaaa"));
        assert_eq!(false, is_very_nice_word("aaasss"));
        assert_eq!(false, is_very_nice_word("uurcxstgmygtbstg"));
        assert_eq!(false, is_very_nice_word("ieodomkazucvgmuy"));
    }
}