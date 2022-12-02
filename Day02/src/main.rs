use std::fs;

fn main() {
    let outcome_encoding:[[u32;3];3] = [[3,6,0],
                                        [0,3,6],
                                        [6,0,3]];
    
    let contents = fs::read_to_string("input.txt").expect("Error reading file");
    let split_contents = contents.lines();

    let score = split_contents.fold(0u32, |n,x| {
        let mut chars = x.chars();
        let opponent_move = chars.next().unwrap() as u32 - 65;
        chars.next();
        let player_move = chars.next().unwrap() as u32 - 88;

        println!("{opponent_move} {player_move}");

        return n + outcome_encoding[opponent_move as usize][player_move as usize] + player_move + 1;
    });
    println!("Final score: {score}");
}
