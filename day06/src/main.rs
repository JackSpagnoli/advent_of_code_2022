use std::fs;
use std::collections::HashSet;

fn main() {
    assert!(length_to_packet_start("test_1_input.txt") == 7);
    assert!(length_to_packet_start("test_2_input.txt") == 5);
    assert!(length_to_packet_start("test_3_input.txt") == 6);
    assert!(length_to_packet_start("test_4_input.txt") == 10);
    assert!(length_to_packet_start("test_5_input.txt") == 11);
    println!("{}", length_to_packet_start("input.txt"));

    assert!(length_to_distinct_sequence("test_1_input.txt", 14) == 19);
    assert!(length_to_distinct_sequence("test_2_input.txt", 14) == 23);
    assert!(length_to_distinct_sequence("test_3_input.txt", 14) == 23);
    assert!(length_to_distinct_sequence("test_4_input.txt", 14) == 29);
    assert!(length_to_distinct_sequence("test_5_input.txt", 14) == 26);
    println!("{}", length_to_distinct_sequence("input.txt", 14));
}

fn length_to_distinct_sequence(file: &str, length: usize) -> u32 {
    let input_contents = fs::read_to_string(file).expect("Error reading file");
    let binding = input_contents.lines().collect::<Vec<&str>>();
    let mut line = binding.get(0).unwrap().chars();

    let mut i: u32 = length as u32;
    while line.size_hint().1.unwrap() >= length {
        let mut next_four = line.clone().take(length).collect::<Vec<char>>();

        let mut seen = HashSet::new();

        next_four.retain(|x: &char| {
            let is_seen = seen.contains(x);
            seen.insert(*x);
            return !is_seen;
        });
        if next_four.len() == length {
            return i;
        }

        i += 1;
        line.next();
    }
    return 0;
}

fn length_to_packet_start(file: &str) -> u32 {
    return length_to_distinct_sequence(file, 4);
}