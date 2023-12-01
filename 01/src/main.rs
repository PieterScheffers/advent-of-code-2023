use std::fs::read_to_string;

fn main() {
    let lines = read_lines("input.txt");
    let lines: Vec<&str> = lines.iter().map(|s| s as &str).collect();
    // println!("{:?}", lines);

    let result = add_number_strings(lines);
    println!("{}", result);
}

pub fn get_first_and_last_digits(str: &str) -> u32 {
    let chars: Vec<_> = str.chars().collect();

    let mut first_number: char = '0';
    let mut last_number: char = '0';

    let mut last_five_chars: Vec<char> = vec![];

    for ch in chars {
        last_five_chars.push(ch);
        last_five_chars = last_five_chars
            .iter()
            .rev()
            .take(5)
            .rev()
            .map(|&x| x)
            .collect();

        let matched = match_number(&last_five_chars, true);

        if ch.is_numeric() {
            first_number = ch;
            break;
        } else if matched.is_ok() {
            first_number = matched.unwrap();
            break;
        }
    }

    // println!("first_number {}", first_number);

    let reversed_chars: Vec<_> = str.chars().rev().collect();
    let mut rev_last_five_chars: Vec<char> = vec![];

    for ch in reversed_chars {
        rev_last_five_chars.push(ch);
        rev_last_five_chars = rev_last_five_chars
            .iter()
            .rev()
            .take(5)
            .rev()
            .map(|&x| x)
            .collect();

        let last_five_chars_turned_back = rev_last_five_chars.iter().rev().map(|&x| x).collect();
        let matched = match_number(&last_five_chars_turned_back, false);

        if ch.is_numeric() {
            last_number = ch;
            break;
        } else if matched.is_ok() {
            last_number = matched.unwrap();
            break;
        }
    }

    // println!("last_number {}", last_number);

    let mut number_string = String::from(first_number);
    number_string.push(last_number);

    number_string
        .trim()
        .parse()
        .expect("Found number characters should be valid numbers")
}

fn match_number(last_five_chars_input: &Vec<char>, is_left_to_right: bool) -> Result<char, String> {
    let last_five_chars = &last_five_chars_input.clone();
    let last_four_chars: Vec<_> = if is_left_to_right {
        last_five_chars.iter().rev().take(4).rev().collect()
    } else {
        last_five_chars.iter().take(4).collect()
    };

    let last_three_chars: Vec<_> = if is_left_to_right {
        last_five_chars.iter().rev().take(3).rev().collect()
    } else {
        last_five_chars.iter().take(3).collect()
    };

    let last_five_chars = &String::from_iter(last_five_chars)[..];
    let last_four_chars = &String::from_iter(last_four_chars)[..];
    let last_three_chars = &String::from_iter(last_three_chars)[..];

    // println!("last_five_chars: {:?}", last_five_chars);
    // println!("last_four_chars: {:?}", last_four_chars);
    // println!("last_three_chars: {:?}", last_three_chars);
    // println!("");

    match &last_three_chars[..] {
        "one" => return Ok('1'),
        "two" => return Ok('2'),
        "six" => return Ok('6'),
        _ => {}
    }

    match &last_four_chars[..] {
        "four" => return Ok('4'),
        "five" => return Ok('5'),
        "nine" => return Ok('9'),
        _ => {}
    }

    match &last_five_chars[..] {
        "three" => return Ok('3'),
        "seven" => return Ok('7'),
        "eight" => return Ok('8'),
        _ => {}
    }

    return Err("Chars not matched".to_string());
}

pub fn add_number_strings(strings: Vec<&str>) -> u32 {
    strings
        .iter()
        .map(|&x| get_first_and_last_digits(x))
        .reduce(|a, b| a + b)
        .expect("Couldn't sum all numbers in u32 vector")
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
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

    #[test]
    fn match_number_test() {
        assert_eq!(match_number(&vec!['o', 'n', 'e'], true).unwrap(), '1');
        assert_eq!(
            match_number(&vec!['k', 'k', 'o', 'n', 'e'], true).unwrap(),
            '1'
        );
        assert_eq!(match_number(&vec!['t', 'w', 'o'], true).unwrap(), '2');
        assert_eq!(
            match_number(&vec!['3', '3', 't', 'w', 'o'], true).unwrap(),
            '2'
        );
        assert_eq!(
            match_number(&vec!['t', 'h', 'r', 'e', 'e'], true).unwrap(),
            '3'
        );
        assert_eq!(match_number(&vec!['f', 'o', 'u', 'r'], true).unwrap(), '4');
        assert_eq!(match_number(&vec!['f', 'i', 'v', 'e'], true).unwrap(), '5');
        assert_eq!(match_number(&vec!['s', 'i', 'x'], true).unwrap(), '6');
        assert_eq!(
            match_number(&vec!['k', '8', 's', 'i', 'x'], true).unwrap(),
            '6'
        );
        assert_eq!(
            match_number(&vec!['s', 'e', 'v', 'e', 'n'], true).unwrap(),
            '7'
        );
        assert_eq!(
            match_number(&vec!['e', 'i', 'g', 'h', 't'], true).unwrap(),
            '8'
        );
        assert_eq!(match_number(&vec!['n', 'i', 'n', 'e'], true).unwrap(), '9');
    }
}
