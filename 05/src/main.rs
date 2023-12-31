use std::cmp;
use std::{collections::HashMap, collections::VecDeque, fs::read_to_string};

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

fn parse_input(raw_input: &String) -> (Vec<i64>, Vec<(&str, Vec<(i64, i64, i64)>)>) {
    let pieces = raw_input
        .split("\n\n")
        .map(|x| x.trim())
        .collect::<Vec<&str>>();

    let seeds: Vec<i64> = {
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
            .collect::<Vec<i64>>()
    };

    let blocks = pieces
        .iter()
        .skip(1)
        .map(|x| parse_block(x))
        .collect::<Vec<(&str, Vec<(i64, i64, i64)>)>>();

    // println!("seeds: {:?}, blocks: {:?}", seeds, blocks);

    (seeds, blocks)
}

fn parse_block(block_str: &str) -> (&str, Vec<(i64, i64, i64)>) {
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

    let mappings: Vec<(i64, i64, i64)> = block_str
        .lines()
        .into_iter()
        .skip(1)
        .map(parse_block_line)
        .collect::<Vec<(i64, i64, i64)>>();

    // println!("name: {}, mappings: {:?}", name, mappings);

    (name, mappings)
}

fn parse_block_line(str: &str) -> (i64, i64, i64) {
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
    blocks: Vec<(&str, Vec<(i64, i64, i64)>)>,
) -> HashMap<String, Vec<(i64, i64, i64)>> {
    blocks
        .iter()
        .fold(HashMap::new(), |mut acc, (name, values)| {
            // acc.insert(name.to_string(), values_to_hashmap(values));
            acc.insert(name.to_string(), values.clone());
            acc
        })
}

fn values_to_hashmap(values: &Vec<(i64, i64, i64)>) -> HashMap<i64, i64> {
    let mut hash = HashMap::new();

    for (dest, source, range) in values {
        for increment in 0..*range {
            hash.insert(source + increment, dest + increment);
        }
    }

    hash
}

fn get_hash_value(hash: &HashMap<String, Vec<(i64, i64, i64)>>, name: String, source: i64) -> i64 {
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
    hash: &HashMap<String, Vec<(i64, i64, i64)>>,
    seed: i64,
) -> (i64, i64, i64, i64, i64, i64, i64, i64) {
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

fn map_slice(
    (dest, source, range): &(i64, i64, i64),
    (slice_source, slice_range): (i64, i64),
) -> (Vec<(i64, i64)>, Vec<(i64, i64)>) {
    let mut new_mapped_slices: Vec<(i64, i64)> = vec![];
    let mut new_slices: Vec<(i64, i64)> = vec![];

    let slice_end = slice_source + slice_range; // not inclusive
    let mapping_end = source + range; // not inclusive

    let slice_source_is_in_mapping = slice_source >= *source && slice_source < mapping_end;
    let slice_end_is_in_mapping = slice_end <= mapping_end && slice_end > *source;

    let mapping_source_is_in_slice = *source >= slice_source && *source < slice_end;
    let mapping_end_is_in_slice = mapping_end <= slice_end && mapping_end > slice_source;

    let increment = dest - source;

    // slice:   xxxx
    // mapping: xxxxxx

    // slice:     xxxx
    // mapping: xxxxxx

    // slice:    xxxx
    // mapping: xxxxxx

    // slice:   xxxxxx
    // mapping:  xxxx

    // slice:    xxxx
    // mapping: xxxxxx
    if slice_source_is_in_mapping && slice_end_is_in_mapping {
        new_mapped_slices.push((slice_source + increment, slice_range));

    // slice:       xxxx => xx | xx
    // mapping: xxxxxx
    } else if slice_source_is_in_mapping && !slice_end_is_in_mapping {
        new_mapped_slices.push((slice_source + increment, mapping_end - slice_source));
        new_slices.push((mapping_end, slice_range - (mapping_end - slice_source)));

    // slice:   xxxx
    // mapping:   xxxxxx
    } else if !slice_source_is_in_mapping && slice_end_is_in_mapping {
        new_slices.push((slice_source, source - slice_source));
        new_mapped_slices.push((*source + increment, slice_range - (source - slice_source)));

    // slice:   xxxxxx
    // mapping:   xx
    } else if mapping_source_is_in_slice && mapping_end_is_in_slice {
        new_slices.push((slice_source, source - slice_source));
        new_mapped_slices.push((*source + increment, *range));
        new_slices.push((mapping_end, slice_range - range - (source - slice_source)));
    } else {
        new_slices.push((slice_source, slice_range));
    }

    (new_mapped_slices, new_slices)
}

fn map_slices(mappings: &Vec<(i64, i64, i64)>, slices: Vec<(i64, i64)>) -> Vec<(i64, i64)> {
    let tuple = mappings.iter().fold((vec![], slices), |acc, mapping| {
        let mut mapped_slices = acc.0;
        let slices_todo = acc.1;

        let mut new_slices_todo: Vec<(i64, i64)> = vec![];

        for slice_todo in slices_todo {
            let (new_mapped_slices, new_slices) = map_slice(&mapping, slice_todo);

            for new_mapped_slice in new_mapped_slices {
                mapped_slices.push(new_mapped_slice);
            }

            for new_slice in new_slices {
                new_slices_todo.push(new_slice);
            }
        }

        (mapped_slices, new_slices_todo)
    });

    let mut mapped_slices = tuple.0;
    let new_slices_todo = tuple.1;

    for new_slice in new_slices_todo {
        mapped_slices.push(new_slice);
    }

    mapped_slices
}

fn chunk_seeds(seeds: Vec<i64>) -> Vec<(i64, i64)> {
    seeds.chunks(2).map(|x| (x[0], x[1])).collect()
}

fn part_one(raw_input: &String) -> i64 {
    let (seeds, blocks) = parse_input(raw_input);
    let hash = convert_blocks_into_hashes(blocks);

    seeds
        .iter()
        .map(|seed| get_all_category_values(&hash, *seed))
        .map(|x| x.7)
        .fold(i64::MAX, |acc, nr| cmp::min(acc, nr))
}

fn part_two(raw_input: &String) -> i64 {
    let (seeds, blocks) = parse_input(raw_input);
    let slices = chunk_seeds(seeds);
    let hash = convert_blocks_into_hashes(blocks);

    let slices = map_slices(hash.get(&String::from("seed-to-soil")).unwrap(), slices);
    let slices = map_slices(
        hash.get(&String::from("soil-to-fertilizer")).unwrap(),
        slices,
    );
    let slices = map_slices(
        hash.get(&String::from("fertilizer-to-water")).unwrap(),
        slices,
    );
    let slices = map_slices(hash.get(&String::from("water-to-light")).unwrap(), slices);
    let slices = map_slices(
        hash.get(&String::from("light-to-temperature")).unwrap(),
        slices,
    );
    let slices = map_slices(
        hash.get(&String::from("temperature-to-humidity")).unwrap(),
        slices,
    );
    let slices = map_slices(
        hash.get(&String::from("humidity-to-location")).unwrap(),
        slices,
    );

    slices
        .iter()
        .fold(i64::MAX, |acc, (source, _)| cmp::min(acc, *source))
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

    #[test]
    fn can_map_slice() {
        // No mapping (to low)
        let mapping: (i64, i64, i64) = (4, 5, 2);
        let slice: (i64, i64) = (1, 2);
        let mapped_slices: Vec<(i64, i64)> = vec![];
        let unmapped_slices: Vec<(i64, i64)> = vec![(1, 2)];

        assert_eq!(map_slice(&mapping, slice), (mapped_slices, unmapped_slices));

        // No mapping (to low)
        let mapping: (i64, i64, i64) = (4, 1, 2);
        let slice: (i64, i64) = (4, 2);
        let mapped_slices: Vec<(i64, i64)> = vec![];
        let unmapped_slices: Vec<(i64, i64)> = vec![(4, 2)];

        assert_eq!(map_slice(&mapping, slice), (mapped_slices, unmapped_slices));

        // Slice within mapping
        let mapping: (i64, i64, i64) = (4, 1, 6); // 1, 2, 3, 4, 5, 6
        let slice: (i64, i64) = (2, 2); // 2, 3
        let mapped_slices: Vec<(i64, i64)> = vec![(5, 2)];
        let unmapped_slices: Vec<(i64, i64)> = vec![];

        assert_eq!(map_slice(&mapping, slice), (mapped_slices, unmapped_slices));

        // Slice overlapping and to the right of the mapping
        let mapping: (i64, i64, i64) = (5, 1, 6); // 1, 2, 3, 4, 5, 6
        let slice: (i64, i64) = (4, 5); // 4, 5, 6, 7, 8
        let mapped_slices: Vec<(i64, i64)> = vec![(8, 3)];
        let unmapped_slices: Vec<(i64, i64)> = vec![(7, 2)];

        assert_eq!(map_slice(&mapping, slice), (mapped_slices, unmapped_slices));

        // Slice overlapping and to the left of the mapping
        let mapping: (i64, i64, i64) = (8, 4, 6); // 4, 5, 6, 7, 8, 9
        let slice: (i64, i64) = (2, 4); // 2, 3, 4, 5
        let mapped_slices: Vec<(i64, i64)> = vec![(8, 2)];
        let unmapped_slices: Vec<(i64, i64)> = vec![(2, 2)];

        assert_eq!(map_slice(&mapping, slice), (mapped_slices, unmapped_slices));

        // Slice overlapping mapping
        let mapping: (i64, i64, i64) = (8, 4, 2); // 4, 5
        let slice: (i64, i64) = (2, 6); // 2, 3, 4, 5, 6, 7
        let mapped_slices: Vec<(i64, i64)> = vec![(8, 2)];
        let unmapped_slices: Vec<(i64, i64)> = vec![(2, 2), (6, 2)];

        assert_eq!(map_slice(&mapping, slice), (mapped_slices, unmapped_slices));
    }
}
