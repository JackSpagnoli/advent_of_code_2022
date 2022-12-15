use std::fs;
use regex::Regex;
use std::cmp::{ max, min };

fn main() {
    assert_eq!(beaconless_spaces("test_input.txt", 10), 26);
    println!("{}", beaconless_spaces("input.txt", 2000000));
}

fn beaconless_spaces(file: &str, row: usize) -> usize {
    let (sensor_coordinates, beacon_coordinates, (min_x, max_x)) = sensor_coordinate_pairs(file);

    let mut spaces: usize = 0;
    for x in min_x..max_x + 1 {
        for i in 0..sensor_coordinates.len() {
            let (coordinates, distance) = sensor_coordinates[i];
            if l1_distance(coordinates, (row as isize, x)) <= distance {
                spaces += 1;
                break;
            }
        }
    }

    for i in 0..beacon_coordinates.len() {
        if beacon_coordinates[i].0 == (row as isize) {
            spaces -= 1;
        }
    }

    return spaces;
}

fn sensor_coordinate_pairs(
    file: &str
) -> (Vec<((isize, isize), isize)>, Vec<(isize, isize)>, (isize, isize)) {
    let contents = fs::read_to_string(file).expect("Error reading file");
    let regex = Regex::new(
        r"Sensor at x=(?P<s_x>-?\d+), y=(?P<s_y>\d+): closest beacon is at x=(?P<b_x>-?\d+), y=(?P<b_y>\d+)"
    ).unwrap();

    let mut sensor_coordinates: Vec<((isize, isize), isize)> = vec!();
    let mut beacon_coordinates: Vec<(isize, isize)> = vec!();

    let mut min_x: isize = isize::MAX;
    let mut max_x: isize = 0;
    let mut max_distance: isize = 0;
    for capture in regex.captures_iter(&contents) {
        let coords: ((isize, isize), (isize, isize)) = (
            (capture["s_y"].parse::<isize>().unwrap(), capture["s_x"].parse::<isize>().unwrap()),
            (capture["b_y"].parse::<isize>().unwrap(), capture["b_x"].parse::<isize>().unwrap()),
        );

        let distance = l1_distance(coords.0, coords.1);
        max_distance = max(max_distance, distance);

        min_x = min(min_x, min(coords.0.1, coords.1.1));
        max_x = max(max_x, max(coords.0.1, coords.1.1));

        sensor_coordinates.push((coords.0, distance));
        if !beacon_coordinates.contains(&coords.1) {
            beacon_coordinates.push(coords.1);
        }
    }

    return (sensor_coordinates, beacon_coordinates, (min_x - max_distance, max_x + max_distance));
}

fn l1_distance((y_1, x_1): (isize, isize), (y_2, x_2): (isize, isize)) -> isize {
    let dy = y_1 - y_2;
    let dx = x_1 - x_2;
    return max(dy, -dy) + max(dx, -dx);
}