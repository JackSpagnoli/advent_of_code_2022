use std::fs;
use pathfinding::prelude::dijkstra;

fn main() {
    assert_eq!(find_shortest_path("test_input.txt"), 31);
    println!("{}", find_shortest_path("input.txt"));
}

fn successors(
    (y, x): (usize, usize),
    height_map: &Vec<Vec<isize>>
) -> Vec<((usize, usize), usize)> {
    if y >= height_map.len() || x >= height_map[0].len() {
        return vec!();
    }
    let mut orthogonal_paths: Vec<(usize, usize)> = vec!();
    if y <= height_map.len() - 2 {
        if
            height_map[y + 1][x] - height_map[y][x] <= 1 
            // &&
            // height_map[y + 1][x] - height_map[y][x] >= -1
        {
            orthogonal_paths.push((y + 1, x));
        }
    }
    if y >= 1 {
        if
            height_map[y - 1][x] - height_map[y][x] <= 1 
            // &&
            // height_map[y - 1][x] - height_map[y][x] >= -1
        {
            orthogonal_paths.push((y - 1, x));
        }
    }
    if x <= height_map[0].len() - 2 {
        if
            height_map[y][x + 1] - height_map[y][x] <= 1 
            // &&
            // height_map[y][x + 1] - height_map[y][x] >= -1
        {
            orthogonal_paths.push((y, x + 1));
        }
    }
    if x >= 1 {
        if
            height_map[y][x - 1] - height_map[y][x] <= 1 
            // &&
            // height_map[y][x - 1] - height_map[y][x] >= -1
        {
            orthogonal_paths.push((y, x - 1));
        }
    }
    // println!("({y},{x}) : {orthogonal_paths:?}");
    return orthogonal_paths
        .into_iter()
        .map(|p| (p, 1))
        .collect();
}

fn find_shortest_path(file: &str) -> usize {
    let (height_map, destination_pos, start_point) = generate_height_map(file);
    let result = dijkstra(
        &start_point,
        |&(y, x)| successors((y, x), &height_map),
        |p| *p == destination_pos
    ).expect("No path");
    return result.1;
}

fn generate_height_map(file: &str) -> (Vec<Vec<isize>>, (usize, usize), (usize, usize)) {
    let contents = fs::read_to_string(file).expect("Error reading file");
    let lines = contents.lines();

    let mut height_map: Vec<Vec<isize>> = vec!();
    let mut end_point: (usize, usize) = (0, 0);
    let mut start_point: (usize, usize) = (0, 0);

    let mut j = 0;
    for line in lines {
        let mut i = 0;
        height_map.push(
            line
                .chars()
                .map(|x| {
                    i += 1;
                    if x == 'E' {
                        end_point = (j, i - 1);
                        return 25;
                    } else if x == 'S' {
                        start_point = (j, i - 1);
                        return 0;
                    }
                    return (x as isize) - 97;
                })
                .collect()
        );
        j += 1;
    }
    return (height_map, end_point, start_point);
}