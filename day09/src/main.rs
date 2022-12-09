use std::fs;

fn main() {
    assert_eq!(distinct_tail_positions("test_input.txt", 2), 13);
    println!("{}", distinct_tail_positions("input.txt", 2));

    assert_eq!(distinct_tail_positions("test_input_2.txt", 10), 36);
    println!("{}", distinct_tail_positions("input.txt", 10));
}

fn distinct_tail_positions(file: &str, knots: usize) -> usize {
    return generate_tail_path(file, knots)
        .iter()
        .fold(vec![(0i32, 0i32)], |mut acc: Vec<(i32, i32)>, pos: &(i32, i32)| {
            // let mut acc_copy = acc.clone();
            for prev_pos in acc.clone().iter() {
                if prev_pos.0 == pos.0 && prev_pos.1 == pos.1 {
                    return acc;
                }
            }
            acc.push(*pos);
            return acc;
        })
        .len();
}

fn process_tail_move(head_pos: (i32, i32), tail_pos: (i32, i32)) -> (i32, i32) {
    let mut tail_move: (i32, i32) = (0, 0);

    let diff: (i32, i32) = (head_pos.0 - tail_pos.0, head_pos.1 - tail_pos.1);

    if diff == (0, 2) {
        tail_move = (0, 1);
    }
    if diff == (0, -2) {
        tail_move = (0, -1);
    }
    if diff == (2, 0) {
        tail_move = (1, 0);
    }
    if diff == (-2, 0) {
        tail_move = (-1, 0);
    }
    if diff == (1, 2) || diff == (2, 2) || diff == (2, 1) {
        tail_move = (1, 1);
    }
    if diff == (-1, -2) || diff == (-2, -2) || diff == (-2, -1) {
        tail_move = (-1, -1);
    }
    if diff == (-1, 2) || diff == (-2, 2) || diff == (-2, 1) {
        tail_move = (-1, 1);
    }
    if diff == (1, -2) || diff == (2, -2) || diff == (2, -1) {
        tail_move = (1, -1);
    }

    return (tail_pos.0 + tail_move.0, tail_pos.1 + tail_move.1);
}

fn generate_tail_path(file: &str, knot_count: usize) -> Vec<(i32, i32)> {
    let moves = generate_moves(file);

    let mut knots: Vec<(i32, i32)> = vec![(0,0);knot_count];

    let mut all_tail_positions: Vec<(i32, i32)> = vec![(0, 0)];

    for next_move in moves {
        knots[0].0 += next_move.0;
        knots[0].1 += next_move.1;

        for i in 0..knot_count - 1 {
            knots[i + 1] = process_tail_move(knots[i], knots[i + 1]);
        }
        all_tail_positions.push(knots[knot_count - 1]);
    }

    return all_tail_positions;
}

fn generate_moves(file: &str) -> Vec<(i32, i32)> {
    let input_contents = fs::read_to_string(file).expect("Error reading file");
    let lines = input_contents.lines();

    let mut moves: Vec<(i32, i32)> = vec!();
    for line in lines {
        let mut split = line.split(" ");
        let (dir, repeats): (&str, i32) = (
            split.next().unwrap(),
            split.next().unwrap().parse::<i32>().unwrap(),
        );
        for _ in 0..repeats {
            match dir {
                "U" => moves.push((1, 0)),
                "D" => moves.push((-1, 0)),
                "R" => moves.push((0, 1)),
                "L" => moves.push((0, -1)),
                &_ => println!("Big yikes on line {line}"),
            }
        }
    }

    return moves;
}