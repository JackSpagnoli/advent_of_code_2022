use std::fs::{ read_to_string };

fn main() {
    assert_eq!(sum_signal_strength("test_input.txt", vec![20, 60, 100, 140, 180, 220]), 13140);
    println!("{}", sum_signal_strength("input.txt", vec![20, 60, 100, 140, 180, 220]));

    assert_eq!(
        draw_image("test_input.txt", 40),
        "##..##..##..##..##..##..##..##..##..##..\n###...###...###...###...###...###...###.\n####....####....####....####....####....\n#####.....#####.....#####.....#####.....\n######......######......######......####\n#######.......#######.......#######.....\n"
    );
    println!("{}", draw_image("input.txt", 40));
}

fn sum_signal_strength(file: &str, times: Vec<usize>) -> isize {
    let instructions: Vec<(bool, isize)> = read_instructions(file);

    let mut cycle: usize = 1;
    let mut x_reg: isize = 1;

    let mut strength: isize = 0;
    for (add, amount) in instructions {
        if add {
            cycle += 1;
            x_reg += amount;
        } else {
            cycle += 1;
        }

        if times.contains(&cycle) {
            strength += (cycle as isize) * x_reg;
        }
    }

    return strength;
}

fn draw_image(file: &str, width: isize) -> String {
    let instructions: Vec<(bool, isize)> = read_instructions(file);

    let mut cycle: isize = 0;
    let mut x_reg: isize = 1;

    let mut image: String = "".to_string();

    for (add, amount) in instructions {
        if cycle % width >= x_reg - 1 && cycle % width <= x_reg + 1 {
            image = image.to_owned() + "#";
        } else {
            image = image.to_owned() + ".";
        }

        if (cycle + 1) % width == 0 && cycle > 1 {
            image = image.to_owned() + "\n";
        }

        cycle += 1;

        if add {
            x_reg += amount;
        }
    }
    return image;
}

fn read_instructions(file: &str) -> Vec<(bool, isize)> {
    let input_contents = read_to_string(file).expect("Error reading file");
    let lines = input_contents.lines();

    let mut instructions: Vec<(bool, isize)> = vec!();

    for line in lines {
        instructions.push((false, 0));
        if line != "noop" {
            let mut split = line.split(" ");
            split.next();
            instructions.push((true, split.next().unwrap().parse::<isize>().unwrap()));
        }
    }

    return instructions;
}