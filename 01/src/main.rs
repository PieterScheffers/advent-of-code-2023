use std::fs::read_to_string;

fn main() {
    part_one();
}

pub fn get_first_and_last_digits(str: &str) -> u32 {
    let chars: Vec<_> = str.chars().collect();

    let mut first_number: char = '0';
    let mut last_number: char = '0';

    for ch in chars {
        if ch.is_numeric() {
            first_number = ch;
            break;
        }
    }

    let reversed_chars: Vec<_> = str.chars().rev().collect();

    for ch in reversed_chars {
        if ch.is_numeric() {
            last_number = ch;
            break;
        }
    }

    let mut number_string = String::from(first_number);
    number_string.push(last_number);

    number_string
        .trim()
        .parse()
        .expect("Found number characters should be valid numbers")
}

pub fn add_number_strings(strings: Vec<&str>) -> u32 {
    strings
        .iter()
        .map(|&x| get_first_and_last_digits(x))
        .reduce(|a, b| a + b)
        .expect("blaga")
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}

fn part_one() {
    let lines = read_lines("input.txt");
    let lines: Vec<&str> = lines.iter().map(|s| s as &str).collect();
    // println!("{:?}", lines);

    let result = add_number_strings(lines);
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_first_and_last_digits_test() {
        // Part one
        assert_eq!(get_first_and_last_digits("1abc2"), 12);
        assert_eq!(get_first_and_last_digits("pqr3stu8vwx"), 38);
        assert_eq!(get_first_and_last_digits("a1b2c3d4e5f"), 15);
        assert_eq!(get_first_and_last_digits("treb7uchet"), 77);

        // Part two
        assert_eq!(get_first_and_last_digits("two1nine"), 29);
        assert_eq!(get_first_and_last_digits("eightwothree"), 83);
        assert_eq!(get_first_and_last_digits("abcone2threexyz"), 13);
        assert_eq!(get_first_and_last_digits("xtwone3four"), 24);
        assert_eq!(get_first_and_last_digits("4nineeightseven2"), 42);
        assert_eq!(get_first_and_last_digits("zoneight234"), 14);
        assert_eq!(get_first_and_last_digits("7pqrstsixteen"), 76);
    }

    #[test]
    fn add_number_strings_test() {
        // Part one
        let numbers = vec!["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"];
        let result = add_number_strings(numbers);
        assert_eq!(result, 142);

        // Part two
        let numbers = vec![
            "two1nine",
            "eightwothree",
            "abcone2threexyz",
            "xtwone3four",
            "4nineeightseven2",
            "zoneight234",
            "7pqrstsixteen",
        ];
        let result = add_number_strings(numbers);
        assert_eq!(result, 281);
    }
}
