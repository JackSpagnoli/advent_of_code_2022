use std::fs;
use std::str::FromStr;

fn main() {
    assert!(overlapping_pairs("test_input.txt") == 2);
    println!("{}", overlapping_pairs("input.txt"));
    assert!(strict_overlapping_pairs("test_input.txt") == 4);
    println!("{}", strict_overlapping_pairs("input.txt"));
}

fn overlapping_pairs(file: &str) -> u32 {
    let contents = fs::read_to_string(file).expect("Error reading file");
    let split_contents = contents.lines();

    return split_contents.fold(0, |n, x| {
        let mut split_line = x.split(",");

        let mut left_range = split_line.next().unwrap().split("-");
        let left_a = <u32 as FromStr>::from_str(left_range.next().unwrap()).unwrap() as u32;
        let left_b = <u32 as FromStr>::from_str(left_range.next().unwrap()).unwrap() as u32;

        let mut right_range = split_line.next().unwrap().split("-");
        let right_a = <u32 as FromStr>::from_str(right_range.next().unwrap()).unwrap() as u32;
        let right_b = <u32 as FromStr>::from_str(right_range.next().unwrap()).unwrap() as u32;

        if (left_a >= right_a && left_b <= right_b) || (left_a <= right_a && left_b >= right_b) {
            return n + 1;
        } else {
            return n;
        }
    });
}

fn strict_overlapping_pairs(file: &str) -> u32 {
    let contents = fs::read_to_string(file).expect("Error reading file");
    let split_contents = contents.lines();

    return split_contents.fold(0, |n, x| {
        let mut split_line = x.split(",");

        let mut left_range = split_line.next().unwrap().split("-");
        let left_a = <u32 as FromStr>::from_str(left_range.next().unwrap()).unwrap() as u32;
        let left_b = <u32 as FromStr>::from_str(left_range.next().unwrap()).unwrap() as u32;

        let mut right_range = split_line.next().unwrap().split("-");
        let right_a = <u32 as FromStr>::from_str(right_range.next().unwrap()).unwrap() as u32;
        let right_b = <u32 as FromStr>::from_str(right_range.next().unwrap()).unwrap() as u32;

        if
            (left_a >= right_a && left_a <= right_b) ||
            (left_b >= right_a && left_b <= right_b) ||
            (right_a >= left_a && right_a <= left_b) ||
            (right_b >= left_a && right_b <= left_b)
        {
            return n + 1;
        } else {
            return n;
        }
    });
}