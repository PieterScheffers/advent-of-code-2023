use std::collections::HashMap;
use std::fs::read_to_string;
use std::hash::{Hash, Hasher};
use std::mem;

#[derive(Debug, Clone)]
struct RouteNode {
    node: String,
    index: usize,
    steps: i64,
}

impl PartialEq for RouteNode {
    fn eq(&self, other: &Self) -> bool {
        self.node == other.node && self.index == other.index
    }
}
impl Eq for RouteNode {}

impl Hash for RouteNode {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.node.hash(state);
        self.index.hash(state);
    }
}

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

    let input = r#"
        LR

        11A = (11B, XXX)
        11B = (XXX, 11Z)
        11Z = (11B, XXX)
        22A = (22B, XXX)
        22B = (22C, 22C)
        22C = (22Z, 22Z)
        22Z = (22B, 22B)
        XXX = (XXX, XXX)
    "#
    .trim()
    .to_string();

    // let input = read_file("input.txt");

    // let result_part_one = part_one(&input);
    // println!("result_part_one: {}", result_part_one);

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

/// Create a hashmap with the 3-character node and the index as key and the number of steps as value
// fn hash_per_node_index_and_steps<'a>(
//     directions: &Vec<&str>,
//     node_map: &HashMap<&str, (&str, &str)>,
// ) -> HashMap<RouteNode, RouteNode> {
//     let mut hash = HashMap::new();

//     for index in 0..directions.len() {
//         for (node, (_left, _right)) in node_map.into_iter() {
//             let source_route_node = RouteNode {
//                 node: String::from(*node),
//                 index: index,
//                 steps: 0,
//             };
//             let dest_route_node = get_next_z_node(&directions, &node_map, source_route_node);

//             hash.insert(source_route_node, dest_route_node);
//         }
//     }

//     hash
// }

fn get_next_z_node(
    directions: &Vec<&str>,
    node_map: &HashMap<&str, (&str, &str)>,
    route_node: &RouteNode,
) -> RouteNode {
    let mut steps = 0i64;
    let mut index = route_node.index;

    let mut node = route_node.node.clone();

    let max_index_direction = directions.len() - 1;

    loop {
        steps += 1;

        let (left, right) = (&node_map)
            .get(node.as_str())
            .expect(&format!("Cannot get key {} from node_map", node));

        let next_direction = directions[index];

        node = match next_direction {
            "L" => left.to_string(),
            "R" => right.to_string(),
            _ => panic!("Cannot determine which side to choose"),
        };

        index = if index >= max_index_direction {
            0
        } else {
            index + 1
        };

        let is_done = node.chars().last().unwrap() == 'Z';

        if is_done {
            break;
        }
    }

    RouteNode { node, index, steps }
}

fn fill_hash_till_next_z_node(
    directions: &Vec<&str>,
    node_map: &HashMap<&str, (&str, &str)>,
    hash: &mut HashMap<RouteNode, RouteNode>,
    start_route_node: &RouteNode,
) -> RouteNode {
    let mut steps = 0i64;
    let mut index = start_route_node.index;

    let mut node = start_route_node.node.clone();

    let max_index_direction = directions.len() - 1;

    let mut last_route_node = start_route_node.clone();
    last_route_node.steps = 0;

    let mut route_nodes = vec![last_route_node];

    loop {
        steps += 1;

        let (left, right) = (&node_map)
            .get(node.as_str())
            .expect(&format!("Cannot get key {} from node_map", node));

        let next_direction = directions[index];

        node = match next_direction {
            "L" => left.to_string(),
            "R" => right.to_string(),
            _ => panic!("Cannot determine which side to choose"),
        };

        index = if index >= max_index_direction {
            0
        } else {
            index + 1
        };

        last_route_node = RouteNode {
            node: node.clone(),
            index: index.clone(),
            steps: steps.clone(),
        };

        println!(
            "New last_route_node: node: {}, index: {}, steps: {}",
            node, index, steps
        );

        let is_done = node.chars().last().unwrap() == 'Z';

        if is_done {
            break;
        } else {
            if let Some(hashed_route_node) = hash.get(&last_route_node) {
                println!(
                    "Found hashed_route_node - node: {}, index: {}, steps: {}, total_steps: {}, calculated_steps: {}",
                    hashed_route_node.node,
                    hashed_route_node.index,
                    hashed_route_node.steps,
                    steps,
                    steps + hashed_route_node.steps
                );
                steps += hashed_route_node.steps;
                break;
            }

            route_nodes.push(last_route_node);
        }
    }

    for route_node in route_nodes {
        let mut new_route_node = last_route_node.clone();
        println!(
            "Calculating steps- node: {}, route_node_steps: {}, steps: {}, calculated_steps: {}",
            route_node.node,
            route_node.steps,
            steps,
            steps - route_node.steps
        );
        new_route_node.steps = steps - route_node.steps;
        hash.insert(route_node, new_route_node);
    }

    last_route_node
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
    let (directions, node_map) = parse_input(&raw_input);

    // let hash = hash_per_node_index_and_steps(&directions, &node_map);
    // println!("hash: {:?}", hash);

    let start_nodes = (&node_map)
        .clone()
        .into_keys()
        .filter(|x| x.chars().last().unwrap() == 'A')
        .collect::<Vec<&str>>();

    println!("start_nodes: {:?}", start_nodes);

    // println!("directions: {:?}", directions);
    // println!("node_map: {:?}", node_map);

    let mut is_done = false;
    let mut route_nodes = start_nodes
        .iter()
        .map(|node| RouteNode {
            node: node.to_string(),
            index: 0,
            steps: 0,
        })
        .collect::<Vec<RouteNode>>();

    let mut hash: HashMap<RouteNode, RouteNode> = HashMap::new();

    // hash.insert(
    //     RouteNode {
    //         node: "11Z".to_string(),
    //         index: 0,
    //         steps: 4,
    //     },
    //     RouteNode {
    //         node: "11Z".to_string(),
    //         index: 0,
    //         steps: 6,
    //     },
    // );
    // hash.insert(
    //     RouteNode {
    //         node: "22Z".to_string(),
    //         index: 1,
    //         steps: 3,
    //     },
    //     RouteNode {
    //         node: "22Z".to_string(),
    //         index: 0,
    //         steps: 6,
    //     },
    // );
    // hash.insert(
    //     RouteNode {
    //         node: "22A".to_string(),
    //         index: 0,
    //         steps: 0,
    //     },
    //     RouteNode {
    //         node: "22Z".to_string(),
    //         index: 1,
    //         steps: 3,
    //     },
    // );
    // hash.insert(
    //     RouteNode {
    //         node: "11Z".to_string(),
    //         index: 0,
    //         steps: 2,
    //     },
    //     RouteNode {
    //         node: "11Z".to_string(),
    //         index: 0,
    //         steps: 4,
    //     },
    // );
    // hash.insert(
    //     RouteNode {
    //         node: "11A".to_string(),
    //         index: 0,
    //         steps: 0,
    //     },
    //     RouteNode {
    //         node: "11Z".to_string(),
    //         index: 0,
    //         steps: 2,
    //     },
    // );

    // hash.insert(
    //     RouteNode {
    //         node: "22C".to_string(),
    //         index: 0,
    //         steps: 2,
    //     },
    //     RouteNode {
    //         node: "22Z".to_string(),
    //         index: 1,
    //         steps: 3,
    //     },
    // );
    // hash.insert(
    //     RouteNode {
    //         node: "11Z".to_string(),
    //         index: 0,
    //         steps: 8,
    //     },
    //     RouteNode {
    //         node: "11Z".to_string(),
    //         index: 0,
    //         steps: 10,
    //     },
    // );
    // hash.insert(
    //     RouteNode {
    //         node: "11A".to_string(),
    //         index: 0,
    //         steps: 0,
    //     },
    //     RouteNode {
    //         node: "11Z".to_string(),
    //         index: 0,
    //         steps: 2,
    //     },
    // );
    // hash.insert(
    //     RouteNode {
    //         node: "11Z".to_string(),
    //         index: 0,
    //         steps: 4,
    //     },
    //     RouteNode {
    //         node: "11Z".to_string(),
    //         index: 0,
    //         steps: 6,
    //     },
    // );
    // hash.insert(
    //     RouteNode {
    //         node: "11Z".to_string(),
    //         index: 0,
    //         steps: 2,
    //     },
    //     RouteNode {
    //         node: "11Z".to_string(),
    //         index: 0,
    //         steps: 4,
    //     },
    // );
    // hash.insert(
    //     RouteNode {
    //         node: "11Z".to_string(),
    //         index: 0,
    //         steps: 6,
    //     },
    //     RouteNode {
    //         node: "11Z".to_string(),
    //         index: 0,
    //         steps: 8,
    //     },
    // );
    // hash.insert(
    //     RouteNode {
    //         node: "11B".to_string(),
    //         index: 1,
    //         steps: 1,
    //     },
    //     RouteNode {
    //         node: "11Z".to_string(),
    //         index: 0,
    //         steps: 2,
    //     },
    // );
    // hash.insert(
    //     RouteNode {
    //         node: "22Z".to_string(),
    //         index: 1,
    //         steps: 3,
    //     },
    //     RouteNode {
    //         node: "22Z".to_string(),
    //         index: 0,
    //         steps: 6,
    //     },
    // );
    // hash.insert(
    //     RouteNode {
    //         node: "22A".to_string(),
    //         index: 0,
    //         steps: 0,
    //     },
    //     RouteNode {
    //         node: "22Z".to_string(),
    //         index: 1,
    //         steps: 3,
    //     },
    // );
    // hash.insert(
    //     RouteNode {
    //         node: "22Z".to_string(),
    //         index: 0,
    //         steps: 9,
    //     },
    //     RouteNode {
    //         node: "22Z".to_string(),
    //         index: 1,
    //         steps: 12,
    //     },
    // );
    // hash.insert(
    //     RouteNode {
    //         node: "22B".to_string(),
    //         index: 1,
    //         steps: 1,
    //     },
    //     RouteNode {
    //         node: "22Z".to_string(),
    //         index: 1,
    //         steps: 3,
    //     },
    // );
    // hash.insert(
    //     RouteNode {
    //         node: "11Z".to_string(),
    //         index: 0,
    //         steps: 10,
    //     },
    //     RouteNode {
    //         node: "11Z".to_string(),
    //         index: 0,
    //         steps: 12,
    //     },
    // );

    let mut iteration = 0i64;

    while !is_done {
        iteration += 1;

        let (lowest_index, lowest_steps) = route_nodes
            .iter()
            .enumerate()
            .reduce(|acc, x| if acc.1.steps > x.1.steps { x } else { acc })
            .expect("Cannot find the RouteNode with the least steps");

        if (iteration % 1000000) == 0 {
            println!(
                "lowest_index: {}, lowest_steps: {:?}, hash keys: {}",
                lowest_index,
                lowest_steps,
                hash.keys().len()
            );
        }

        if let Some(new_route_node) = hash.get(lowest_steps) {
            println!("Got RouteNode in hash: {:?}", lowest_steps);

            let mut route_node = new_route_node.clone();
            route_node.steps += lowest_steps.steps;

            route_nodes[lowest_index] = route_node;
        } else {
            println!("Cannot get RouteNode in hash: {:?}", lowest_steps);
            // panic!("Should not try to calc route");
            let new_route_node =
                fill_hash_till_next_z_node(&directions, &node_map, &mut hash, lowest_steps);
            let mut route_node = new_route_node.clone();
            route_node.steps += lowest_steps.steps;

            hash.insert(lowest_steps.clone(), route_node.clone());
            route_nodes[lowest_index] = route_node;
        }

        let are_all_on_the_same_step = route_nodes
            .iter()
            .map(|x| x.steps)
            .fold(true, |acc, x| acc && x == route_nodes[0].steps);

        let are_all_z = route_nodes.iter().fold(true, |acc, route_node| {
            acc && route_node.node.chars().last().unwrap() == 'Z'
        });

        is_done = are_all_on_the_same_step && are_all_z;
    }

    println!("hash: {:?}", hash);

    route_nodes[0].steps
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

    #[test]
    fn can_solve_part_two() {
        let input = r#"
            LR

            11A = (11B, XXX)
            11B = (XXX, 11Z)
            11Z = (11B, XXX)
            22A = (22B, XXX)
            22B = (22C, 22C)
            22C = (22Z, 22Z)
            22Z = (22B, 22B)
            XXX = (XXX, XXX)
        "#
        .trim()
        .to_string();

        assert_eq!(part_two(&input), 6);
    }

    #[test]
    fn can_use_route_node_as_hash_key() {
        let mut hash: HashMap<RouteNode, i64> = HashMap::new();

        hash.insert(
            RouteNode {
                node: "AAA".to_string(),
                index: 22,
                steps: 45,
            },
            22,
        );

        let route_node = RouteNode {
            node: "AAA".to_string(),
            index: 22,
            steps: 22,
        };
        let value = hash
            .get(&route_node)
            .expect("[can_use_route_node_as_hash_key]: Error cannot get routenode");

        assert_eq!(value, &22i64);
    }
}
