use std::convert::TryInto;
use std::fs::read_to_string;

fn main() {
    let input = r#"
        32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483
    "#
    .trim()
    .to_string();

    // let input = read_file("input.txt");

    let result_part_one = part_one(&input);
    println!("result_part_one: {}", result_part_one);

    let result_part_two = part_two(&input);
    println!("result_part_two: {}", result_part_two);
}

fn read_file(filename: &str) -> String {
    read_to_string(filename).expect(&format!("Should be able to read file {}", filename))
}

fn parse_input(raw_input: &String) -> Vec<([i64; 5], i64)> {
    raw_input.lines().into_iter().map(parse_line).collect()
}

fn parse_line(line: &str) -> ([i64; 5], i64) {
    let mut parts_iter = line.split(" ").map(|x| x.trim()).filter(|x| x != &"");

    let cards = parts_iter
        .next()
        .unwrap()
        .split("")
        .map(|x| x.trim())
        .filter(|x| x != &"")
        .map(map_card_to_number)
        .collect::<Vec<i64>>();

    let bid: i64 = parts_iter
        .next()
        .unwrap()
        .parse()
        .expect("Bid should be a number");

    let cards_array = vector_to_array::<i64, 5>(cards);

    (cards_array, bid)
}

fn map_card_to_number(ch: &str) -> i64 {
    // println!("char: {}", ch);

    match ch {
        "A" => 14,
        "K" => 13,
        "Q" => 12,
        "J" => 11,
        "T" => 10,
        "9" | "8" | "7" | "6" | "5" | "4" | "3" | "2" => {
            ch.parse().expect("Char should be a number")
        }
        _ => panic!("Character cannot be mapped to card number"),
    }
}

fn vector_to_array<T, const N: usize>(v: Vec<T>) -> [T; N] {
    v.try_into()
        .unwrap_or_else(|v: Vec<T>| panic!("Expected a Vec of length {} but it was {}", N, v.len()))
}

fn map_cards_to_type(cards: [i64; 5]) -> i64 {
    // 7: Five of a kind, where all five cards have the same label: AAAAA
    // 6: Four of a kind, where four cards have the same label and one card has a different label: AA8AA
    // 5: Full house, where three cards have the same label, and the remaining two cards share a different label: 23332
    // 4: Three of a kind, where three cards have the same label, and the remaining two cards are each different from any other card in the hand: TTT98
    // 3: Two pair, where two cards share one label, two other cards share a second label, and the remaining card has a third label: 23432
    // 2: One pair, where two cards share one label, and the other three cards have a different label from the pair and each other: A23A4
    // 1: High card, where all cards' labels are distinct: 23456

    // TODO

    1
}

fn part_one(raw_input: &String) -> i64 {
    let sets = parse_input(raw_input);

    println!("sets: {:?}", sets);
    2
}

fn part_two(raw_input: &String) -> i64 {
    2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_solve_part_one() {
        let input = r#"
            32T3K 765
            T55J5 684
            KK677 28
            KTJJT 220
            QQQJA 483
        "#
        .trim()
        .to_string();

        let result = part_one(&input);
        assert_eq!(result, 6440);
    }
}
