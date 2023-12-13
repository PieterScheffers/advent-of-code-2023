use std::cmp::Ordering::{Equal, Greater, Less};
use std::collections::HashMap;
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

    let input = read_file("input.txt");

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

fn map_cards_to_type(cards: &[i64; 5]) -> i64 {
    // 7: Five of a kind, where all five cards have the same label: AAAAA
    // 6: Four of a kind, where four cards have the same label and one card has a different label: AA8AA
    // 5: Full house, where three cards have the same label, and the remaining two cards share a different label: 23332
    // 4: Three of a kind, where three cards have the same label, and the remaining two cards are each different from any other card in the hand: TTT98
    // 3: Two pair, where two cards share one label, two other cards share a second label, and the remaining card has a third label: 23432
    // 2: One pair, where two cards share one label, and the other three cards have a different label from the pair and each other: A23A4
    // 1: High card, where all cards' labels are distinct: 23456

    let grouped: HashMap<i64, i64> = cards.iter().fold(HashMap::new(), |mut acc, c| {
        let def: i64 = 0;
        let i: i64 = *acc.get(c).unwrap_or(&def);
        acc.insert(*c, i + 1);
        acc
    });

    // println!("grouped: {:?}", grouped);

    let def: (i64, i64) = (0, 0);
    let longest = (&grouped).into_iter().fold(def, |acc, card| {
        if *card.1 > acc.1 {
            (*card.0, *card.1)
        } else {
            acc
        }
    });

    // println!("longest: {:?}", longest);

    let second_longest = grouped
        .into_iter()
        .filter(|(card, _)| card != &longest.0)
        .fold(def, |acc, card| {
            if card.1 > acc.1 {
                (card.0, card.1)
            } else {
                acc
            }
        });

    // println!("second_longest: {:?}", second_longest);

    if longest.1 == 5 {
        return 7;
    } else if longest.1 == 4 {
        return 6;
    } else if longest.1 == 3 && second_longest.1 == 2 {
        return 5;
    } else if longest.1 == 3 {
        return 4;
    } else if longest.1 == 2 && second_longest.1 == 2 {
        return 3;
    } else if longest.1 == 2 {
        return 2;
    }

    1
}

fn sort_sets(sets: &mut Vec<([i64; 5], i64)>) -> &mut Vec<([i64; 5], i64)> {
    let hand_type_cmp = |l: &([i64; 5], i64), r: &([i64; 5], i64)| {
        map_cards_to_type(&l.0).cmp(&map_cards_to_type(&r.0))
    };

    let first_cmp = |l: &([i64; 5], i64), r: &([i64; 5], i64)| l.0[0].cmp(&r.0[0]);
    let second_cmp = |l: &([i64; 5], i64), r: &([i64; 5], i64)| l.0[1].cmp(&r.0[1]);
    let third_cmp = |l: &([i64; 5], i64), r: &([i64; 5], i64)| l.0[2].cmp(&r.0[2]);
    let fourth_cmp = |l: &([i64; 5], i64), r: &([i64; 5], i64)| l.0[3].cmp(&r.0[3]);
    let fifth_cmp = |l: &([i64; 5], i64), r: &([i64; 5], i64)| l.0[4].cmp(&r.0[4]);

    sets.sort_by(|a, b| {
        let result = hand_type_cmp(&a, &b);
        if result != Equal {
            return result;
        }

        let result = first_cmp(&a, &b);
        if result != Equal {
            return result;
        }

        let result = second_cmp(&a, &b);
        if result != Equal {
            return result;
        }

        let result = third_cmp(&a, &b);
        if result != Equal {
            return result;
        }

        let result = fourth_cmp(&a, &b);
        if result != Equal {
            return result;
        }

        fifth_cmp(&a, &b)
    });

    sets
}

fn part_one(raw_input: &String) -> i64 {
    let mut sets: Vec<([i64; 5], i64)> = parse_input(raw_input);
    // println!("sets: {:?}", sets);

    let sets = sort_sets(&mut sets);
    // println!("sets: {:?}", sets);

    sets.iter().enumerate().fold(0, |acc, (index, (_, bid))| {
        acc + ((i64::try_from(index).expect("Cannot convert usize to i64") + 1) * bid)
    })
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

    #[test]
    fn can_map_cards_to_type() {
        assert_eq!(map_cards_to_type(&[5, 2, 10, 3, 13]), 1);
        assert_eq!(map_cards_to_type(&[3, 2, 10, 3, 13]), 2);
        assert_eq!(map_cards_to_type(&[13, 13, 6, 7, 7]), 3);
        assert_eq!(map_cards_to_type(&[10, 5, 5, 11, 5]), 4);
        assert_eq!(map_cards_to_type(&[3, 10, 10, 3, 3]), 5);
        assert_eq!(map_cards_to_type(&[3, 3, 10, 3, 3]), 6);
        assert_eq!(map_cards_to_type(&[3, 3, 3, 3, 3]), 7);
    }
}
