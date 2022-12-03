use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Error reading file");
    let split_contents = contents.lines();
    let mut totals: Vec<u32> = vec![0];
    for line in split_contents {
        if line == "" {
            totals.push(0);
        } else {
            let length = totals.len() - 1;
            totals[length] += line.parse::<u32>().unwrap();
        }
    }
    totals.sort();
    let tail_total = totals[totals.len() - 1] + totals[totals.len() - 2] + totals[totals.len() - 3];

    println!("{tail_total}")
}
