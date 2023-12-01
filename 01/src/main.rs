fn main() {
    let a = String::from("hello");

    println!("Hello, world!");
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(get_first_and_last_digits("1abc2"), 12);
        assert_eq!(get_first_and_last_digits("pqr3stu8vwx"), 38);
        assert_eq!(get_first_and_last_digits("a1b2c3d4e5f"), 15);
        assert_eq!(get_first_and_last_digits("treb7uchet"), 77);
    }
}
