use std::fs;

fn main() {
    let mut trees: Vec<Vec<u32>> = vec!();
    read_trees(&mut trees, "test_input.txt");
    assert_eq!(count_visible_trees(&trees), 21);
    assert_eq!(max_senic_score(&trees), 8);

    trees = vec!();
    read_trees(&mut trees, "input.txt");
    println!("{}", count_visible_trees(&trees));
    println!("{}", max_senic_score(&trees));
}

fn read_trees(trees: &mut Vec<Vec<u32>>, file: &str) {
    let input_contents = fs::read_to_string(file).expect("Error reading file");
    let lines = input_contents.lines();

    for line in lines {
        let mut line_mut = vec!();
        for tree in line.chars() {
            line_mut.push(tree.to_digit(10).unwrap());
        }
        trees.push(line_mut);
    }
}

fn count_visible_trees(trees: &Vec<Vec<u32>>) -> u32 {
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

fn max_senic_score(trees: &Vec<Vec<u32>>) -> u32 {
    let mut max_score: u32 = 0;
    for y in 0..trees.len() {
        for x in 0..trees[y].len() {
            let score = scenic_score(&trees, y, x);
            if score > max_score {
                max_score = score;
            }
        }
    }
    return max_score;
}

fn scenic_score(trees: &Vec<Vec<u32>>, y: usize, x: usize) -> u32 {
    let h = trees[y][x];
    let mut score: u32 = 1;

    let mut k: u32 = 0;
    for i in x + 1..trees[y].len() {
        k += 1;
        if trees[y][i] >= h {
            break;
        }
    }
    score *= k;
    k = 0;

    for i in (0..x).rev() {
        k += 1;
        if trees[y][i] >= h {
            break;
        }
    }
    score *= k;
    k = 0;

    for j in y + 1..trees.len() {
        k += 1;
        if trees[j][x] >= h {
            break;
        }
    }
    score *= k;
    k = 0;

    for j in (0..y).rev() {
        k += 1;
        if trees[j][x] >= h {
            break;
        }
    }
    score *= k;

    return score;
}