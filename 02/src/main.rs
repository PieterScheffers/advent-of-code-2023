use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const MAX_RED_CUBES: u32 = 12;
const MAX_GREEN_CUBES: u32 = 13;
const MAX_BLUE_CUBES: u32 = 14;

fn main() {
    let mut total_part_one: u32 = 0;
    let mut total_part_two: u32 = 0;

    // File hosts.txt must exist in the current path
    if let Ok(lines) = read_lines("input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line_result in lines {
            if let Ok(line) = line_result {
                if is_line_valid(&line) {
                    total_part_one += get_game_number_from_line(&line);
                }

                total_part_two += get_power_of_minimum_cubes(&line);
            }
        }
    }

    println!("Total part 1: {}", total_part_one);
    println!("Total part 2: {}", total_part_two);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn is_line_valid(line: &str) -> bool {
    let cube_grabs = get_hash_vector_from_line(line);

    is_game_valid(cube_grabs)
}

fn get_game_number_from_line(line: &str) -> u32 {
    let mut split_by_colon = line.split(":").take(1);

    split_by_colon
        .next()
        .unwrap()
        .split(" ")
        .skip(1)
        .take(1)
        .next()
        .unwrap()
        .parse()
        .expect("Game should be an integer")
}

fn get_hash_vector_from_line(line: &str) -> Vec<HashMap<&str, u32>> {
    let mut split_by_colon = line.split(":").take(2).skip(1);

    // 14 green, 3 red, 16 blue; 3 blue, 6 green; 12 green, 6 blue, 2 red
    // [
    //     {
    //         green: 14
    //         red: 3
    //         blue: 16
    //     },
    //     {
    //         blue: 3,
    //         green: 6
    //     },
    //     {
    //         green: 12,
    //         blue: 6,
    //         red: 2
    //     }
    // ]
    split_by_colon
        .next()
        .unwrap()
        .split(";")
        .map(|x| x.trim())
        .map(|x| parse_grab(x))
        .collect()
}

fn parse_grab(cubes_str: &str) -> HashMap<&str, u32> {
    let cubes: Vec<&str> = cubes_str.split(",").map(|x| x.trim()).collect();

    let mut cube_map = HashMap::new();

    for cube_str in cubes {
        let qty_and_color: Vec<&str> = cube_str.split(" ").map(|x| x.trim()).collect();
        let quantity: u32 = qty_and_color[0]
            .parse()
            .expect("Quantity should be a number");
        let color = qty_and_color[1];

        cube_map.insert(color, quantity);
    }

    cube_map
}

fn is_game_valid(grabs: Vec<HashMap<&str, u32>>) -> bool {
    grabs
        .into_iter()
        .fold(true, |acc, grab| acc && is_grab_valid(grab))
}

fn is_grab_valid(grab: HashMap<&str, u32>) -> bool {
    grab.into_iter().fold(true, |acc, color_quantity_tuple| {
        acc && is_color_valid(color_quantity_tuple)
    })
}

fn is_color_valid((color, quantity): (&str, u32)) -> bool {
    let result = match color {
        "red" => MAX_RED_CUBES >= quantity,
        "green" => MAX_GREEN_CUBES >= quantity,
        "blue" => MAX_BLUE_CUBES >= quantity,
        _ => return true,
    };

    // println!("Color: {}, qty: {}, result: {}", color, quantity, result);

    result
}

fn get_power_of_minimum_cubes(line: &str) -> u32 {
    let minimum_cubes = get_minimum_cubes(line);

    let mut power: u32 = 1;

    for (_, quantity) in minimum_cubes.into_iter() {
        power = power * quantity;
    }

    power
}

fn get_minimum_cubes(line: &str) -> HashMap<&str, u32> {
    let cube_grabs = get_hash_vector_from_line(line);

    let mut minimum_cubes: HashMap<&str, u32> = HashMap::new();

    for cube_grab in cube_grabs {
        for (color, quantity) in cube_grab.into_iter() {
            let current_minimum = minimum_cubes.get(color).unwrap_or_else(|| &0);

            if current_minimum < &quantity {
                minimum_cubes.insert(color, quantity);
            }
        }
    }

    minimum_cubes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_minimum_cubes_test() {
        let line = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let minimums = get_minimum_cubes(line);

        assert_eq!(minimums.contains_key("blue"), true);
        assert_eq!(minimums.contains_key("red"), true);
        assert_eq!(minimums.contains_key("green"), true);

        assert_eq!(minimums.get("blue").unwrap().to_owned(), 6);
        assert_eq!(minimums.get("red").unwrap().to_owned(), 4);
        assert_eq!(minimums.get("green").unwrap().to_owned(), 2);

        let line = "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue";
        let minimums = get_minimum_cubes(line);

        assert_eq!(minimums.contains_key("blue"), true);
        assert_eq!(minimums.contains_key("red"), true);
        assert_eq!(minimums.contains_key("green"), true);

        assert_eq!(minimums.get("blue").unwrap().to_owned(), 4);
        assert_eq!(minimums.get("red").unwrap().to_owned(), 1);
        assert_eq!(minimums.get("green").unwrap().to_owned(), 3);

        let line = "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red";
        let minimums = get_minimum_cubes(line);

        assert_eq!(minimums.contains_key("blue"), true);
        assert_eq!(minimums.contains_key("red"), true);
        assert_eq!(minimums.contains_key("green"), true);

        assert_eq!(minimums.get("blue").unwrap().to_owned(), 6);
        assert_eq!(minimums.get("red").unwrap().to_owned(), 20);
        assert_eq!(minimums.get("green").unwrap().to_owned(), 13);

        let line = "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red";
        let minimums = get_minimum_cubes(line);

        assert_eq!(minimums.contains_key("blue"), true);
        assert_eq!(minimums.contains_key("red"), true);
        assert_eq!(minimums.contains_key("green"), true);

        assert_eq!(minimums.get("blue").unwrap().to_owned(), 15);
        assert_eq!(minimums.get("red").unwrap().to_owned(), 14);
        assert_eq!(minimums.get("green").unwrap().to_owned(), 3);

        let line = "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let minimums = get_minimum_cubes(line);

        assert_eq!(minimums.contains_key("blue"), true);
        assert_eq!(minimums.contains_key("red"), true);
        assert_eq!(minimums.contains_key("green"), true);

        assert_eq!(minimums.get("blue").unwrap().to_owned(), 2);
        assert_eq!(minimums.get("red").unwrap().to_owned(), 6);
        assert_eq!(minimums.get("green").unwrap().to_owned(), 3);
    }

    #[test]
    fn get_power_of_minimum_cubes_test() {
        let line = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        assert_eq!(get_power_of_minimum_cubes(line), 48);

        let line = "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue";
        assert_eq!(get_power_of_minimum_cubes(line), 12);

        let line = "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red";
        assert_eq!(get_power_of_minimum_cubes(line), 1560);

        let line = "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red";
        assert_eq!(get_power_of_minimum_cubes(line), 630);

        let line = "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(get_power_of_minimum_cubes(line), 36);
    }
}
