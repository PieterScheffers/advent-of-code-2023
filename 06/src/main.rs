use std::fs::read_to_string;

fn main() {
    let input = r#"
        Time:      7  15   30
        Distance:  9  40  200
    "#
    .trim()
    .to_string();

    let input = read_file("input.txt");

    let result_part_one = part_one(&input);
    println!("result_part_one: {}", result_part_one);

    let result_part_two = part_two(&input);
    println!("result_part_two: {}", result_part_two);

    println!("Hello, world!");
}

fn read_file(filename: &str) -> String {
    read_to_string(filename).expect(&format!("Should be able to read file {}", filename))
}

fn parse_input(raw_input: &String) -> Vec<(i64, i64)> {
    let first_line = raw_input
        .lines()
        .into_iter()
        .take(1)
        .reduce(|acc, x| x)
        .expect("Cannot get first line")
        .split(" ")
        .map(|x| x.trim())
        .filter(|x| x != &"")
        .collect::<Vec<&str>>();

    let second_line = raw_input
        .lines()
        .into_iter()
        .skip(1)
        .take(1)
        .reduce(|acc, x| x)
        .expect("Cannot get second line")
        .split(" ")
        .map(|x| x.trim())
        .filter(|x| x != &"")
        .collect::<Vec<&str>>();

    let mut races: Vec<(i64, i64)> = vec![];

    for (index, time) in first_line.iter().enumerate() {
        if index < 1 {
            continue;
        }

        let time = time.parse().expect("Cannot parse time");

        let distance = second_line
            .get(index)
            .expect(&format!("Cannot get distance with index {}", index))
            .parse()
            .expect("Cannot parse distance");

        races.push((time, distance));
    }

    races
}

fn part_one(raw_input: &String) -> i64 {
    let races = parse_input(raw_input);

    println!("races: {:?}", races);

    2
}

fn part_two(raw_input: &String) -> i64 {
    2
}
