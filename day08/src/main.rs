use std::fs;

fn main() {
    assert_eq!(count_visible_trees("test_input.txt"), 21);
    println!("{}", count_visible_trees("input.txt"));
}

fn count_visible_trees(file: &str) -> u32 {
    let input_contents = fs::read_to_string(file).expect("Error reading file");
    let lines = input_contents.lines();

    let mut trees: Vec<Vec<u32>> = vec![];

    for line in lines {
        let mut line_mut = vec!();
        for tree in line.chars() {
            line_mut.push(tree.to_digit(10).unwrap());
        }
        trees.push(line_mut);
    }

    let mut visible_trees: u32 = 0;
    for j in 0..trees.len() {
        for i in 0..trees[0].len() {
            if is_visible(&trees, i, j) {
                visible_trees += 1;
            }
        }
    }

    return visible_trees;
}

fn is_visible(trees: &Vec<Vec<u32>>, x: usize, y: usize) -> bool {
    let h = trees[y][x];

    if x == 0 || y == 0 || x == trees[0].len() - 1 || y == trees.len() - 1 {
        return true;
    }

    let mut visibility: bool = true;
    for i in x + 1..trees[y].len() {
        if trees[y][i] >= h {
            visibility = false;
        }
    }
    if visibility {
        return visibility;
    }

    visibility = true;
    for i in 0..x {
        if trees[y][i] >= h {
            visibility = false;
        }
    }
    if visibility {
        return visibility;
    }

    visibility = true;
    for j in y + 1..trees.len() {
        if trees[j][x] >= h {
            visibility = false;
        }
    }
    if visibility {
        return visibility;
    }

    visibility = true;
    for j in 0..y {
        if trees[j][x] >= h {
            visibility = false;
        }
    }
    if visibility {
        return visibility;
    }

    return false;
}