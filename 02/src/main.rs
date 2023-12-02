// use std::fs::read_to_string;

// fn main() {
//     let lines = read_lines("input.txt");
//     let lines: Vec<&str> = lines.iter().map(|s| s as &str).collect();
// }

// fn read_lines(filename: &str) -> Vec<String> {
//     read_to_string(filename)
//         .unwrap() // panic on possible file-reading errors
//         .lines() // split the string into an iterator of string slices
//         .map(String::from) // make each slice into a string
//         .collect() // gather them together into a vector
// }

// fn check_line_validates() {
//     // split by :
//     // split on ;
//     // split on ,
//     // split on space
//     // left side = number, right side = color

// }

use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const MAX_RED_CUBES: u32 = 12;
const MAX_GREEN_CUBES: u32 = 13;
const MAX_BLUE_CUBES: u32 = 14;

fn main() {
    let mut total: u32 = 0;

    // File hosts.txt must exist in the current path
    if let Ok(lines) = read_lines("input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line_result in lines {
            if let Ok(line) = line_result {
                if is_line_valid(&line) {
                    total += get_game_number_from_line(&line);
                }
            }
        }
    }

    println!("Total: {}", total)
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
    let cube_grabs: Vec<HashMap<&str, u32>> = split_by_colon
        .next()
        .unwrap()
        .split(";")
        .map(|x| x.trim())
        .map(|x| parse_grab(x))
        .collect();

    // println!("g: {}", game);

    // let mut index: u32 = 0;

    // for cube_grab in cube_grabs {
    //     for (color, quantity) in cube_grab.into_iter() {
    //         println!("Index:{}, {} / {}", index, color, quantity);

    //         // match &color[..] {
    //         //     "red" => return Ok('1'),
    //         //     "green" => return Ok('2'),
    //         //     "blue" => return Ok('6'),
    //         //     _ => {}
    //         // }
    //     }

    //     index = index + 1;
    // }

    // println!("");

    let is_valid = is_game_valid(cube_grabs);

    // println!("IsValid: {}", is_valid);

    is_valid
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
