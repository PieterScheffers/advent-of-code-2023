use std::fs::read_to_string;

// Point = (x, y)

#[derive(Debug)]
struct Cell {
    x: usize,
    y: usize,
    ch: char,
}

#[derive(Debug)]
struct MachineNumber {
    cells: Vec<Cell>,
}

impl MachineNumber {
    fn get_value(&self) -> u32 {
        let chars: Vec<String> = self.cells.iter().map(|x| x.ch.to_string()).collect();
        chars.join("").parse().expect("Value should be a number")
    }

    fn has_adjacent_symbol(&self, board: &Vec<Vec<char>>) -> bool {
        self.cells.iter().fold(false, |acc, cell| {
            acc || has_adjacent_symbol(board, (cell.x, cell.y))
        })
    }
}

fn main() {
    // let board_string = r#"
    //     467..114..
    //     ...*......
    //     ..35..633.
    //     ......#...
    //     617*......
    //     .....+.58.
    //     ..592.....
    //     ......755.
    //     ...$.*....
    //     .664.598..
    // "#
    // .trim()
    // .to_string();
    // let result_part_one = part_one(board_string);

    let result_part_one = part_one(read_file("input.txt"));
    println!("result_part_one: {}", result_part_one);
}

fn read_file(filename: &str) -> String {
    read_to_string(filename).expect(&format!("Should be able to read file {}", filename))
}

pub fn parse_input(result: String) -> Vec<Vec<char>> {
    result.lines().map(|s| s.trim().chars().collect()).collect()
}

fn get_numbers(board: &Vec<Vec<char>>) -> Vec<MachineNumber> {
    let mut numbers: Vec<MachineNumber> = vec![];

    for (y, line) in board.iter().enumerate() {
        let mut machine_number = MachineNumber { cells: vec![] };

        for (x, ch) in line.iter().enumerate() {
            // println!("x:{}, y:{}, char:{}", x, y, ch);

            if ch.is_numeric() {
                let cell = Cell {
                    ch: ch.clone(),
                    x: x.clone(),
                    y: y.clone(),
                };
                machine_number.cells.push(cell)
            } else {
                if machine_number.cells.len() > 0 {
                    numbers.push(machine_number);
                    machine_number = MachineNumber { cells: vec![] }
                }
            }
        }

        if machine_number.cells.len() > 0 {
            numbers.push(machine_number);
            machine_number = MachineNumber { cells: vec![] }
        }
    }

    numbers
}

fn is_symbol(char: char) -> bool {
    if char.is_numeric() {
        return false;
    }

    if char == '.' {
        return false;
    }

    true
}

fn get_adjacent_cells(board: &Vec<Vec<char>>, (x, y): (usize, usize)) -> Vec<(usize, usize)> {
    let max_x = board[0].len() - 1;
    let max_y = board.len() - 1;

    let mut coordinates: Vec<(usize, usize)> = vec![];

    // x x x
    // x o x
    // x x x

    if y > 0 && x > 0 {
        coordinates.push((x - 1, y - 1))
    }

    if y > 0 {
        coordinates.push((x, y - 1))
    }

    if y > 0 && x < max_x {
        coordinates.push((x + 1, y - 1))
    }

    if y < max_y && x > 0 {
        coordinates.push((x - 1, y + 1))
    }

    if y < max_y {
        coordinates.push((x, y + 1))
    }

    if y < max_y && x < max_x {
        coordinates.push((x + 1, y + 1))
    }

    if x > 0 {
        coordinates.push((x - 1, y))
    }

    if x < max_x {
        coordinates.push((x + 1, y))
    }

    coordinates
}

fn has_adjacent_symbol(board: &Vec<Vec<char>>, (x, y): (usize, usize)) -> bool {
    let cells: Vec<(usize, usize)> = get_adjacent_cells(board, (x, y));

    let error_message = &format!("Should be able to get cells x:{}, y:{}", x, y);

    for (x, y) in cells {
        let ch = board
            .get(y)
            .expect(error_message)
            .get(x)
            .expect(error_message);

        if is_symbol(*ch) {
            return true;
        }
    }

    false
}

fn part_one(result: String) -> u32 {
    let board = parse_input(result);
    let machine_numbers = get_numbers(&board);

    // let n = machine_numbers
    //     .iter()
    //     .find(|x| x.get_value() == 467)
    //     .expect("Cannot find 467 number");

    // println!(
    //     "has_adjacent_symbol: {}",
    //     has_adjacent_symbol(&board, (3, 0))
    // );

    // println!(
    //     "{:?}, {}, {}",
    //     n,
    //     n.get_value(),
    //     n.has_adjacent_symbol(&board)
    // );

    let numbers: Vec<u32> = machine_numbers
        .iter()
        .filter(|x| x.has_adjacent_symbol(&board))
        .map(|x| x.get_value())
        .collect();

    numbers.iter().fold(0, |acc, num| acc + num)
}

// Parse input
// get all numbers with coordinates
//

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse_input() {
        let board_string = r#"
            467..114..
            ...*......
            ..35..633.
            ......#...
            617*......
            .....+.58.
            ..592.....
            ......755.
            ...$.*....
            .664.598..
        "#
        .trim()
        .to_string();

        let parsed_input = parse_input(board_string);

        let test_board: Vec<Vec<char>> = vec![
            vec!['4', '6', '7', '.', '.', '1', '1', '4', '.', '.'],
            vec!['.', '.', '.', '*', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '3', '5', '.', '.', '6', '3', '3', '.'],
            vec!['.', '.', '.', '.', '.', '.', '#', '.', '.', '.'],
            vec!['6', '1', '7', '*', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '+', '.', '5', '8', '.'],
            vec!['.', '.', '5', '9', '2', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '7', '5', '5', '.'],
            vec!['.', '.', '.', '$', '.', '*', '.', '.', '.', '.'],
            vec!['.', '6', '6', '4', '.', '5', '9', '8', '.', '.'],
        ];

        assert_eq!(parsed_input, test_board);
    }

    #[test]
    fn get_first_and_last_digits_test() {
        let test_board: Vec<Vec<char>> = vec![
            vec!['4', '6', '7', '.', '.', '1', '1', '4', '.', '.'],
            vec!['.', '.', '.', '*', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '3', '5', '.', '.', '6', '3', '3', '.'],
            vec!['.', '.', '.', '.', '.', '.', '#', '.', '.', '.'],
            vec!['6', '1', '7', '*', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '+', '.', '5', '8', '.'],
            vec!['.', '.', '5', '9', '2', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '7', '5', '5', '.'],
            vec!['.', '.', '.', '$', '.', '*', '.', '.', '.', '.'],
            vec!['.', '6', '6', '4', '.', '5', '9', '8', '.', '.'],
        ];

        assert_eq!(
            get_adjacent_cells(&test_board, (5, 5)),
            vec![
                (4, 4),
                (5, 4),
                (6, 4),
                (4, 6),
                (5, 6),
                (6, 6),
                (4, 5),
                (6, 5)
            ]
        );

        assert_eq!(
            get_adjacent_cells(&test_board, (0, 0)),
            vec![(0, 1), (1, 1), (1, 0)]
        );

        assert_eq!(
            get_adjacent_cells(&test_board, (9, 9)),
            vec![(8, 8), (9, 8), (8, 9)]
        );
    }

    #[test]
    fn can_get_numbers() {
        let board_string = r#"
            467..114..
            ...*......
            ..35..633.
            ......#...
            617*......
            .....+.58.
            ..592.....
            ......755.
            ...$.*....
            .664.598..
        "#
        .trim()
        .to_string();

        let board = parse_input(board_string);
        let machine_numbers = get_numbers(&board);

        assert_eq!(machine_numbers.len(), 10);

        let numbers: Vec<u32> = machine_numbers.iter().map(|x| x.get_value()).collect();

        let valid_numbers: Vec<u32> = machine_numbers
            .iter()
            .filter(|x| x.has_adjacent_symbol(&board))
            .map(|x| x.get_value())
            .collect();

        assert_eq!(
            numbers,
            vec![467, 114, 35, 633, 617, 58, 592, 755, 664, 598]
        );

        assert_eq!(valid_numbers, vec![467, 35, 633, 617, 592, 755, 664, 598]);
    }

    #[test]
    fn can_solve_part_one() {
        let board_string = r#"
            467..114..
            ...*......
            ..35..633.
            ......#...
            617*......
            .....+.58.
            ..592.....
            ......755.
            ...$.*....
            .664.598..
        "#
        .trim()
        .to_string();

        let result_part_one = part_one(board_string);

        assert_eq!(result_part_one, 4361);
    }

    #[test]
    fn can_check_is_symbol() {
        assert_eq!(is_symbol('*'), true);
        assert_eq!(is_symbol('#'), true);
        assert_eq!(is_symbol('+'), true);
        assert_eq!(is_symbol('$'), true);
        assert_eq!(is_symbol('.'), false);
        assert_eq!(is_symbol('1'), false);
        assert_eq!(is_symbol('2'), false);
        assert_eq!(is_symbol('3'), false);
        assert_eq!(is_symbol('4'), false);
        assert_eq!(is_symbol('5'), false);
        assert_eq!(is_symbol('6'), false);
        assert_eq!(is_symbol('7'), false);
        assert_eq!(is_symbol('8'), false);
        assert_eq!(is_symbol('9'), false);
    }
}
