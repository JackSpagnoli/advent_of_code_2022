use std::fs;
use std::cmp::max;

#[test]
fn surface_area_correctly_calculated() {
    assert_eq!(surface_area("test_input.txt"), 64);
}

#[test]
fn interior_correctly_filled() {
    assert_eq!(exterior_surface_area("test_input.txt"), 58);
}

fn main() {
    println!("Part 1: {}", surface_area("input.txt"));
    println!("Part 2: {}", exterior_surface_area("input.txt"));
}

fn generate_map(file: &str, map: &mut Vec<Vec<Vec<bool>>>) {
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
}

fn count_surface_area(map: &Vec<Vec<Vec<bool>>>) -> usize {
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

fn surface_area(file: &str) -> usize {
    let mut map: Vec<Vec<Vec<bool>>> = vec!();
    generate_map(file, &mut map);

    return count_surface_area(&map);
}

fn exterior_surface_area(file: &str) -> usize {
    let mut map: Vec<Vec<Vec<bool>>> = vec!();
    generate_map(file, &mut map);
    fill_interiors(&mut map);

    return count_surface_area(&map);
}

fn fill_interiors(map: &mut Vec<Vec<Vec<bool>>>) {
    let mut checked: Vec<Vec<Vec<bool>>> = vec!();

    for z in 0..map.len() {
        let mut temp_2: Vec<Vec<bool>> = vec!();
        for y in 0..map[z].len() {
            let mut temp_1: Vec<bool> = vec!();
            for x in 0..map[z][y].len() {
                temp_1.push(map[z][y][x]);
            }
            temp_2.push(temp_1);
        }
        checked.push(temp_2);
    }

    for z in 0..map.len() {
        for y in 0..map[z].len() {
            for x in 0..map[z][y].len() {
                if !checked[z][y][x] {
                    let (neighbours, interior) = flood(&map, z, y, x);
                    if interior {
                        for (z, y, x) in neighbours {
                            map[z][y][x] = true;
                            checked[z][y][x] = true;
                        }
                    } else {
                        for (z, y, x) in neighbours {
                            checked[z][y][x] = true;
                        }
                    }
                }
            }
        }
    }
}

fn flood(
    map: &Vec<Vec<Vec<bool>>>,
    z: usize,
    y: usize,
    x: usize
) -> (Vec<(usize, usize, usize)>, bool) {
    let mut neighbours: Vec<(usize, usize, usize)> = vec![(z, y, x)];
    let is_interior = recur(&map, &mut neighbours, z, y, x);
    return (neighbours, is_interior);
}

fn recur(
    map: &Vec<Vec<Vec<bool>>>,
    neighbours: &mut Vec<(usize, usize, usize)>,
    z: usize,
    y: usize,
    x: usize
) -> bool {
    if map[z][y][x] {
        return true;
    }
    let mut is_interior = true;
    if z < map.len() - 1 {
        if !neighbours.contains(&(z + 1, y, x)) {
            neighbours.push((z + 1, y, x));
            if !recur(&map, neighbours, z + 1, y, x) {
                is_interior = false;
            }
        }
    }
    if y < map[z].len() - 1 {
        if !neighbours.contains(&(z, y + 1, x)) {
            neighbours.push((z, y + 1, x));
            if !recur(&map, neighbours, z, y + 1, x) {
                is_interior = false;
            }
        }
    }
    if x < map[z][y].len() - 1 {
        if !neighbours.contains(&(z, y, x + 1)) {
            neighbours.push((z, y, x + 1));
            if !recur(&map, neighbours, z, y, x + 1) {
                is_interior = false;
            }
        }
    }
    if z > 0 {
        if !neighbours.contains(&(z - 1, y, x)) {
            neighbours.push((z - 1, y, x));
            if !recur(&map, neighbours, z - 1, y, x) {
                is_interior = false;
            }
        }
    }
    if y > 0 {
        if !neighbours.contains(&(z, y - 1, x)) {
            neighbours.push((z, y - 1, x));
            if !recur(&map, neighbours, z, y - 1, x) {
                is_interior = false;
            }
        }
    }
    if x > 0 {
        if !neighbours.contains(&(z, y, x - 1)) {
            neighbours.push((z, y, x - 1));
            if !recur(&map, neighbours, z, y, x - 1) {
                is_interior = false;
            }
        }
    }

    if
        z == 0 ||
        y == 0 ||
        x == 0 ||
        z == map.len() - 1 ||
        y == map[z].len() - 1 ||
        x == map[z][y].len() - 1
    {
        is_interior = false;
    }

    return is_interior;
}