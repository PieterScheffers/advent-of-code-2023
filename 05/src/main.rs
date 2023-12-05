use std::cmp;
use std::{collections::HashMap, fs::read_to_string};

fn main() {
    let input = r#"
        seeds: 79 14 55 13

        seed-to-soil map:
        50 98 2
        52 50 48

        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15

        fertilizer-to-water map:
        49 53 8
        0 11 42
        42 0 7
        57 7 4

        water-to-light map:
        88 18 7
        18 25 70

        light-to-temperature map:
        45 77 23
        81 45 19
        68 64 13

        temperature-to-humidity map:
        0 69 1
        1 0 69

        humidity-to-location map:
        60 56 37
        56 93 4
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

fn parse_input(raw_input: &String) -> (Vec<u64>, Vec<(&str, Vec<(u64, u64, u64)>)>) {
    let pieces = raw_input
        .split("\n\n")
        .map(|x| x.trim())
        .collect::<Vec<&str>>();

    let seeds: Vec<u64> = {
        let seed_str = *pieces
            .first()
            .unwrap()
            .split(":")
            .map(|x| x.trim())
            .skip(1)
            .take(1)
            .collect::<Vec<&str>>()
            .first()
            .unwrap();

        seed_str
            .split(" ")
            .map(|x| x.trim())
            .filter(|x| *x != "")
            .map(|x| x.parse().expect("Seed should be a number"))
            .collect::<Vec<u64>>()
    };

    let blocks = pieces
        .iter()
        .skip(1)
        .map(|x| parse_block(x))
        .collect::<Vec<(&str, Vec<(u64, u64, u64)>)>>();

    // println!("seeds: {:?}, blocks: {:?}", seeds, blocks);

    (seeds, blocks)
}

fn parse_block(block_str: &str) -> (&str, Vec<(u64, u64, u64)>) {
    let lines = block_str.lines();

    let name = *lines
        .into_iter()
        .take(1)
        .collect::<Vec<&str>>()
        .first()
        .unwrap()
        .split(" ")
        .map(|x| x.trim())
        .take(1)
        .collect::<Vec<&str>>()
        .first()
        .unwrap();

    let mappings: Vec<(u64, u64, u64)> = block_str
        .lines()
        .into_iter()
        .skip(1)
        .map(parse_block_line)
        .collect::<Vec<(u64, u64, u64)>>();

    // println!("name: {}, mappings: {:?}", name, mappings);

    (name, mappings)
}

fn parse_block_line(str: &str) -> (u64, u64, u64) {
    let mut iter = str
        .split(" ")
        .filter(|y| *y != "")
        .map(|y| y.parse().expect("Block number should be an integer"));

    (
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
    )
}

fn convert_blocks_into_hashes(
    blocks: Vec<(&str, Vec<(u64, u64, u64)>)>,
) -> HashMap<String, Vec<(u64, u64, u64)>> {
    blocks
        .iter()
        .fold(HashMap::new(), |mut acc, (name, values)| {
            // acc.insert(name.to_string(), values_to_hashmap(values));
            acc.insert(name.to_string(), values.clone());
            acc
        })
}

fn values_to_hashmap(values: &Vec<(u64, u64, u64)>) -> HashMap<u64, u64> {
    let mut hash = HashMap::new();

    for (dest, source, range) in values {
        for increment in 0..*range {
            hash.insert(source + increment, dest + increment);
        }
    }

    hash
}

fn get_hash_value(hash: &HashMap<String, Vec<(u64, u64, u64)>>, name: String, source: u64) -> u64 {
    let values = hash
        .get(&name)
        .expect(&format!("Cannot get hash for name {}", name));

    for (dest, src, range) in values {
        // println!(
        //     "source: {}, dest: {}, src: {}, range: {}, cmp::max: {}, {} {}",
        //     source,
        //     dest,
        //     src,
        //     range,
        //     cmp::max(src + range - 1, src.clone()),
        //     source >= src.clone(),
        //     source <= cmp::max(src + range - 1, src.clone())
        // );

        if source >= src.clone() && source <= cmp::max(src + range - 1, src.clone()) {
            let increment = source - src;
            return dest + increment;
        }
    }

    // println!("");

    source
}

/**
 * Seed 79, soil 81, fertilizer 81, water 81, light 74, temperature 78, humidity 78, location 82
 * (seed, soil, fertilizer, water, light, temperature, humidity, location)
 */
fn get_all_category_values(
    hash: &HashMap<String, Vec<(u64, u64, u64)>>,
    seed: u64,
) -> (u64, u64, u64, u64, u64, u64, u64, u64) {
    let soil = get_hash_value(hash, String::from("seed-to-soil"), seed);
    let fertilizer = get_hash_value(hash, String::from("soil-to-fertilizer"), soil);
    let water = get_hash_value(hash, String::from("fertilizer-to-water"), fertilizer);
    let light = get_hash_value(hash, String::from("water-to-light"), water);
    let temperature = get_hash_value(hash, String::from("light-to-temperature"), light);
    let humidity = get_hash_value(hash, String::from("temperature-to-humidity"), temperature);
    let location = get_hash_value(hash, String::from("humidity-to-location"), humidity);

    (
        seed,
        soil,
        fertilizer,
        water,
        light,
        temperature,
        humidity,
        location,
    )
}

fn part_one(raw_input: &String) -> u64 {
    let (seeds, blocks) = parse_input(raw_input);

    // println!("{:?}", seeds);

    let hash = convert_blocks_into_hashes(blocks);

    seeds
        .iter()
        .map(|seed| get_all_category_values(&hash, *seed))
        .map(|x| x.7)
        .fold(u64::MAX, |acc, nr| cmp::min(acc, nr))
}

fn part_two(raw_input: &String) -> u64 {
    2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_get_the_correct_category_values() {
        let input = r#"
            seeds: 79 14 55 13

            seed-to-soil map:
            50 98 2
            52 50 48

            soil-to-fertilizer map:
            0 15 37
            37 52 2
            39 0 15

            fertilizer-to-water map:
            49 53 8
            0 11 42
            42 0 7
            57 7 4

            water-to-light map:
            88 18 7
            18 25 70

            light-to-temperature map:
            45 77 23
            81 45 19
            68 64 13

            temperature-to-humidity map:
            0 69 1
            1 0 69

            humidity-to-location map:
            60 56 37
            56 93 4
        "#
        .trim()
        .to_string();

        let (seeds, blocks) = parse_input(&input);
        let hash = convert_blocks_into_hashes(blocks);

        assert_eq!(seeds, vec![79, 14, 55, 13]);

        // Seed 79, soil 81, fertilizer 81, water 81, light 74, temperature 78, humidity 78, location 82.
        // Seed 14, soil 14, fertilizer 53, water 49, light 42, temperature 42, humidity 43, location 43.
        // Seed 55, soil 57, fertilizer 57, water 53, light 46, temperature 82, humidity 82, location 86.
        // Seed 13, soil 13, fertilizer 52, water 41, light 34, temperature 34, humidity 35, location 35.
        assert_eq!(
            get_all_category_values(&hash, 79),
            (79, 81, 81, 81, 74, 78, 78, 82)
        );

        assert_eq!(
            get_all_category_values(&hash, 14),
            (14, 14, 53, 49, 42, 42, 43, 43)
        );

        assert_eq!(
            get_all_category_values(&hash, 55),
            (55, 57, 57, 53, 46, 82, 82, 86)
        );

        assert_eq!(
            get_all_category_values(&hash, 13),
            (13, 13, 52, 41, 34, 34, 35, 35)
        );

        // assert_eq!(parsed_input, test_board);
    }
}
