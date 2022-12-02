use std::fs;

fn main() {
    let outcome_encoding:[[u32;3];3] = [[3,6,0],
                                        [0,3,6],
                                        [6,0,3]];
    
                                        //opp_move r,p,s
                                        //desired_outcome l,d,w
    let player_move_encoding:[[u32;3];3] = [[2,0,1],
                                            [0,1,2],
                                            [1,2,0]];

    let contents = fs::read_to_string("input.txt").expect("Error reading file");
    let split_contents = contents.lines();

    let score = split_contents.fold(0u32, |n,x| {
        let mut chars = x.chars();
        let opponent_move = chars.next().unwrap() as u32 - 65;
        chars.next();
        let desired_outcome = chars.next().unwrap() as u32 - 88;

        return n + (desired_outcome * 3) + player_move_encoding[opponent_move as usize][desired_outcome as usize] + 1;
    });
    println!("Final score: {score}");
}
