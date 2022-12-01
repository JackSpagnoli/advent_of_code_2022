use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Error reading file");
    let split_contents = contents.lines();
    let mut max: u32 = 0;
    let mut current: u32 = 0;
    for line in split_contents {
        if line == "" {
            if current > max {
                max = current;
            }
            current = 0;
        } else {
            current += line.parse::<u32>().unwrap();
        }
    }
    println!("{max}");
}
