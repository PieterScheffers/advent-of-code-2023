use std::collections::HashMap;
use std::fs::read_to_string;

fn main() {
    // let input = r#"
    //     RL

    //     AAA = (BBB, CCC)
    //     BBB = (DDD, EEE)
    //     CCC = (ZZZ, GGG)
    //     DDD = (DDD, DDD)
    //     EEE = (EEE, EEE)
    //     GGG = (GGG, GGG)
    //     ZZZ = (ZZZ, ZZZ)
    // "#
    // .trim()
    // .to_string();

    // let input = r#"
    //     LLR

    //     AAA = (BBB, BBB)
    //     BBB = (AAA, ZZZ)
    //     ZZZ = (ZZZ, ZZZ)
    // "#
    // .trim()
    // .to_string();

    let input = read_file("input.txt");

    let result_part_one = part_one(&input);
    println!("result_part_one: {}", result_part_one);

    let result_part_two = part_two(&input);
    println!("result_part_two: {}", result_part_two);
}

fn read_file(filename: &str) -> String {
    read_to_string(filename).expect(&format!("Should be able to read file {}", filename))
}

fn parse_input(raw_input: &String) -> (Vec<&str>, HashMap<&str, (&str, &str)>) {
    let directions = raw_input
        .lines()
        .take(1)
        .next()
        .expect("Cannot get first line of input")
        .split("")
        .map(|x| x.trim())
        .filter(|x| x != &"")
        .collect::<Vec<&str>>();

    let iter = raw_input.lines().skip(2).map(parse_line);

    let mut node_map: HashMap<&str, (&str, &str)> = HashMap::new();

    for (key, left, right) in iter {
        node_map.insert(key, (left, right));
    }

    (directions, node_map)
}

fn parse_line(line: &str) -> (&str, &str, &str) {
    let parts = line
        .split(['=', '(', ')', ','])
        .map(|x| x.trim())
        .filter(|x| x != &"")
        .collect::<Vec<&str>>();

    (parts[0], parts[1], parts[2])
}

fn part_one(raw_input: &String) -> i64 {
    let (directions, node_map) = parse_input(&raw_input);

    // println!("directions: {:?}", directions);
    // println!("node_map: {:?}", node_map);

    let mut steps = 0i64;
    let mut key = "AAA";
    let mut next_directions = directions.clone();

    loop {
        // println!(
        //     "steps: {}, Key: {}, next_directions: {:?}",
        //     steps, key, next_directions
        // );

        if key == "ZZZ" {
            break;
        }

        if next_directions.is_empty() {
            next_directions = directions.clone();
        }

        let next_direction = next_directions.remove(0);

        steps += 1;

        let sides = node_map
            .get(key)
            .expect(&format!("Cannot get key {} from node_map", key));

        key = match next_direction {
            "L" => sides.0,
            "R" => sides.1,
            _ => panic!("Cannot determine which side to choose"),
        }
    }

    steps
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
            RL

            AAA = (BBB, CCC)
            BBB = (DDD, EEE)
            CCC = (ZZZ, GGG)
            DDD = (DDD, DDD)
            EEE = (EEE, EEE)
            GGG = (GGG, GGG)
            ZZZ = (ZZZ, ZZZ)
        "#
        .trim()
        .to_string();

        assert_eq!(part_one(&input), 2);

        let input = r#"
            LLR

            AAA = (BBB, BBB)
            BBB = (AAA, ZZZ)
            ZZZ = (ZZZ, ZZZ)
        "#
        .trim()
        .to_string();

        assert_eq!(part_one(&input), 6);
    }
}
