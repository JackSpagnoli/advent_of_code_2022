use std::fs;
use std::collections::HashSet;

fn main() {
    assert!(length_to_packet_start("test_1_input.txt") == 7);
    assert!(length_to_packet_start("test_2_input.txt") == 5);
    assert!(length_to_packet_start("test_3_input.txt") == 6);
    assert!(length_to_packet_start("test_4_input.txt") == 10);
    assert!(length_to_packet_start("test_5_input.txt") == 11);
    println!("{}", length_to_packet_start("input.txt"));
}

fn length_to_packet_start(file: &str) -> u32 {
    let input_contents = fs::read_to_string(file).expect("Error reading file");
    let binding = input_contents.lines().collect::<Vec<&str>>();
    let mut line = binding.get(0).unwrap().chars();

    let mut i: u32 = 4;
    while line.size_hint().1.unwrap() >= 4 {
        let mut next_four = line.clone().take(4).collect::<Vec<char>>();

        let mut seen = HashSet::new();

        next_four.retain(|x: &char| {
            let is_seen = seen.contains(x);
            seen.insert(*x);
            return !is_seen;
        });
        if next_four.len() == 4 {
            return i;
        }

        i += 1;
        line.next();
    }
    return 0;
}