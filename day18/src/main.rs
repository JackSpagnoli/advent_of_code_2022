use std::fs;
use std::cmp::max;

#[test]
fn surface_area_correctly_calculated() {
    assert_eq!(surface_area("test_input.txt"), 64);
}

fn main() {
    println!("{}", surface_area("input.txt"));
}

fn surface_area(file: &str) -> usize {
    let contents = fs::read_to_string(file).expect("Error reading file");

    let mut coordinates: Vec<[usize; 3]> = vec!();
    let mut max_x_y_z: [usize; 3] = [0, 0, 0];
    for line in contents.lines() {
        let mut split_line = line.split(",");
        let next_x_y_z = [
            split_line.next().unwrap().parse::<usize>().unwrap(),
            split_line.next().unwrap().parse::<usize>().unwrap(),
            split_line.next().unwrap().parse::<usize>().unwrap(),
        ];
        for j in 0..3 {
            max_x_y_z[j] = max(max_x_y_z[j], next_x_y_z[j]);
        }
        coordinates.push(next_x_y_z);
    }

    let mut map: Vec<Vec<Vec<bool>>> = vec!();

    for _ in 0..=max_x_y_z[0] + 1 {
        let mut temp_2: Vec<Vec<bool>> = vec!();
        for _ in 0..=max_x_y_z[1] + 1 {
            let mut temp_1: Vec<bool> = vec!();
            for _ in 0..=max_x_y_z[2] + 1 {
                temp_1.push(false);
            }
            temp_2.push(temp_1);
        }
        map.push(temp_2);
    }

    for cube in coordinates {
        map[cube[0]][cube[1]][cube[2]] = true;
    }

    let mut surface_area: usize = 0;

    for z in 0..map.len() - 1 {
        for y in 0..map[z].len() - 1 {
            for x in 0..map[z][y].len() - 1 {
                if map[z][y][x] {
                    surface_area += 6;

                    if z != 0 {
                        if map[z - 1][y][x] {
                            surface_area -= 1;
                        }
                    }
                    if y != 0 {
                        if map[z][y - 1][x] {
                            surface_area -= 1;
                        }
                    }
                    if x != 0 {
                        if map[z][y][x - 1] {
                            surface_area -= 1;
                        }
                    }
                    if map[z + 1][y][x] {
                        surface_area -= 1;
                    }
                    if map[z][y + 1][x] {
                        surface_area -= 1;
                    }
                    if map[z][y][x + 1] {
                        surface_area -= 1;
                    }
                }
            }
        }
    }

    return surface_area;
}