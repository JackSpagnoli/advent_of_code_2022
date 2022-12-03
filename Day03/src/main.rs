use std::fs;
use substring::Substring;

fn main() {
    assert!(sum_priorities("test_input.txt") == 157);
    println!("{}", sum_priorities("input.txt"));
}

fn sum_priorities(file: &str) -> u32 {
    let contents = fs::read_to_string(file).expect("Error reading file");
    let split_contents = contents.lines();

    let score = split_contents.fold(0u32, |n, x| {
        let mut left_occurances: [u32; 52] = [0; 52];
        let mut right_occurances: [u32; 52] = [0; 52];
        let mut left_substring = x.substring(0, x.len() / 2).chars();
        let mut right_substring = x.substring(x.len() / 2, x.len()).chars();

        while left_substring.size_hint().0 > 0 {
            let mut left_char = (left_substring.next().unwrap() as usize) - 64;
            if left_char > 26 {
                left_char -= 32;
            } else {
                left_char += 26;
            }
            let mut right_char = (right_substring.next().unwrap() as usize) - 64;
            if right_char > 26 {
                right_char -= 32;
            } else {
                right_char += 26;
            }

            left_occurances[left_char - 1] += 1;
            right_occurances[right_char - 1] += 1;
        }

        let mut score_change: usize = 0;

        for i in 0..52 {
            if left_occurances[i] > 0 {
                if right_occurances[i] > 0 {
                    score_change += i + 1;
                }
            }
        }
        return n + (score_change as u32);
    });
    return score;
}