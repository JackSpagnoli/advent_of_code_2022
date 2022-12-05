use std::fs;
use regex::Regex;

fn main() {
    assert!(process_moves("test_initial_setup.txt", "test_moves.txt", false) == "CMZ");
    println!("{}", process_moves("initial_setup.txt", "moves.txt", false));
    assert!(process_moves("test_initial_setup.txt", "test_moves.txt", true) == "MCD");
    println!("{}", process_moves("initial_setup.txt", "moves.txt", true));
}

fn process_moves(initial: &str, moves: &str, multiple_pickup: bool) -> String {
    let initial_setup_contents = fs::read_to_string(initial).expect("Error reading file");
    let split_initial_setup_contents = initial_setup_contents.lines();

    let moves_contents = fs::read_to_string(moves).expect("Error reading file");
    let split_moves_contents = moves_contents.lines();

    let mut stacks: Vec<Vec<char>> = split_initial_setup_contents
        .map(|x| x.chars().collect())
        .collect();

    let regex = Regex::new(
        r"move (?P<number>\d+) from (?P<source>\d+) to (?P<destination>\d+)"
    ).unwrap();
    let moves: Vec<[usize; 3]> = split_moves_contents
        .map(
            |x| -> [usize; 3] {
                let captures = regex.captures(x).unwrap();
                return [
                    captures["number"].parse::<usize>().unwrap(),
                    captures["source"].parse::<usize>().unwrap(),
                    captures["destination"].parse::<usize>().unwrap(),
                ];
            }
        )
        .collect();

    for instruction in moves {
        if !multiple_pickup {
            for _ in 0..instruction[0] {
                let moved_crate = stacks[instruction[1] - 1].pop().unwrap();
                stacks[instruction[2] - 1].push(moved_crate);
            }
        } else {
            let mut temp_stack: Vec<char> = vec!();
            for _ in 0..instruction[0] {
                temp_stack.push(stacks[instruction[1] - 1].pop().unwrap());
            }
            for _ in 0..instruction[0] {
                stacks[instruction[2] - 1].push(temp_stack.pop().unwrap());
            }
        }
    }

    return stacks
        .iter()
        .map(|x| String::from(x.clone().pop().unwrap()))
        .collect::<Vec<String>>()
        .join("");
}