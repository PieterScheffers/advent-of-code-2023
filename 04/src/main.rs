use std::cmp;
use std::collections::{HashMap, VecDeque};
use std::fs::read_to_string;

fn main() {
    // let input = r#"
    //     Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    //     Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
    //     Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
    //     Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
    //     Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
    //     Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
    // "#
    // .trim()
    // .to_string();

    let input = read_file("input.txt");

    let result_part_one = part_one(&input);
    println!("result_part_one: {}", result_part_one);

    let result_part_two = part_two(&input);
    println!("result_part_two: {}", result_part_two);
}

fn part_one(raw_input: &String) -> u32 {
    let scratch_cards = parse_input(raw_input);

    scratch_cards
        .iter()
        .map(|scratch_card| {
            let number_of_winning_numbers: u32 =
                u32::try_from(get_winning_numbers(&scratch_card).len())
                    .expect("Cannot convert usize to u32");

            if number_of_winning_numbers < 1 {
                return 0;
            } else if number_of_winning_numbers == 1 {
                return 1;
            }

            let base: u32 = 2;
            let num = base.pow(number_of_winning_numbers - 1);

            // println!(
            //     "scratch_card: {:?}, number_of_winning_numbers: {}, num: {}",
            //     scratch_card, number_of_winning_numbers, num
            // );

            num
        })
        .fold(0, |acc, num| acc + num)
}

fn part_two(raw_input: &String) -> u32 {
    let scratch_card_hash = scratch_card_hashmap(parse_input(raw_input));
    let max_game: u32 =
        u32::try_from(scratch_card_hash.len()).expect("Cannot convert usize to u32"); // Hash index not zero based

    let mut scratch_cards_todo: VecDeque<(u32, Vec<u32>, Vec<u32>)> =
        VecDeque::from(parse_input(raw_input));
    let mut scratch_cards_done: Vec<(u32, Vec<u32>, Vec<u32>)> = vec![];

    while let Some(scratch_card) = scratch_cards_todo.pop_front() {
        let number_of_winning_numbers: u32 =
            u32::try_from(get_winning_numbers(&scratch_card).len())
                .expect("Cannot convert usize to u32");

        if number_of_winning_numbers > 0 {
            let next_game: u32 = scratch_card.0 + 1;
            let last_game_plus_one: u32 =
                cmp::min(scratch_card.0 + number_of_winning_numbers + 1, max_game + 1);

            let card_numbers_to_add = next_game..last_game_plus_one;

            // println!(
            //     "part_two scratch_card: {:?} number_of_winning_numbers: {}, card_numbers_to_add: {:?}",
            //     scratch_card, number_of_winning_numbers, card_numbers_to_add
            // );

            for card_number in card_numbers_to_add {
                // println!("card_number: {}", card_number);
                let new_scratch_card_option = scratch_card_hash.get(&card_number);

                if new_scratch_card_option.is_some() {
                    let new_scratch_card = new_scratch_card_option.unwrap().clone();
                    scratch_cards_todo.push_back(new_scratch_card);
                }
            }
        }

        scratch_cards_done.push(scratch_card);
    }

    u32::try_from(scratch_cards_done.len()).expect("Cannot convert usize to u32")
}

fn read_file(filename: &str) -> String {
    read_to_string(filename).expect(&format!("Should be able to read file {}", filename))
}

// Game number
// numbers
// game numbers
// (gameNr: u32, numbers: Vec<u32>, game_numbers: Vec<u32>)

fn parse_input(result: &String) -> Vec<(u32, Vec<u32>, Vec<u32>)> {
    result.lines().map(|s| parse_line(s.trim())).collect()
}

fn parse_line(str: &str) -> (u32, Vec<u32>, Vec<u32>) {
    let card_number: u32 = {
        let split_by_colon = str.split(":").map(|s| s.trim());

        let card_str = split_by_colon.take(1).collect::<Vec<&str>>();
        let card_parts = card_str
            .first()
            .expect("Split by colon should have a first part")
            .split(" ")
            .filter(|x| x.to_string() != "")
            .map(|s| s.trim())
            .collect::<Vec<&str>>();

        card_parts
            .get(1)
            .expect("Card string should have a second element")
            .parse()
            .expect("card number should be a number")
    };

    let (winning_numbers, game_numbers) = {
        let split_by_colon = str.split(":").map(|s| s.trim());

        let winning_and_game_numbers_str = split_by_colon.skip(1).take(1).collect::<Vec<&str>>();
        let winning_and_game_numbers = winning_and_game_numbers_str
            .first()
            .expect("Split by colon should have a second part")
            .split("|")
            .map(|s| s.trim())
            .collect::<Vec<&str>>();

        let winning_numbers = winning_and_game_numbers
            .first()
            .expect("winning_and_game_numbers_str should have a first part")
            .split(" ")
            .filter(|x| x.to_string() != "")
            .map(|s| s.trim().parse().expect("Winning numbers should be numbers"))
            .collect::<Vec<u32>>();

        let game_numbers = winning_and_game_numbers
            .get(1)
            .expect("winning_and_game_numbers_str should have a second part")
            .split(" ")
            .filter(|x| x.to_string() != "")
            .map(|s| s.trim().parse().expect("Game numbers should be numbers"))
            .collect::<Vec<u32>>();

        (winning_numbers, game_numbers)
    };

    // println!(
    //     "card_number {}, winning_numbers: {:?}, game_numbers: {:?}",
    //     card_number, winning_numbers, game_numbers
    // );

    (card_number, winning_numbers, game_numbers)
}

fn scratch_card_hashmap(
    scratch_cards: Vec<(u32, Vec<u32>, Vec<u32>)>,
) -> HashMap<u32, (u32, Vec<u32>, Vec<u32>)> {
    let mut hash = HashMap::new();

    for scratch_card in scratch_cards {
        hash.insert(scratch_card.0, scratch_card);
    }

    hash
}

fn get_winning_numbers((_, winning_numbers, game_numbers): &(u32, Vec<u32>, Vec<u32>)) -> Vec<u32> {
    winning_numbers
        .iter()
        .filter(|x| game_numbers.contains(x))
        .map(|x| *x)
        .collect::<Vec<u32>>()
}
