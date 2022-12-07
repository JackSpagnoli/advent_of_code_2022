use std::fs;
use std::collections::HashMap;

fn main() {
    let mut directories: Vec<u128> = vec!();
    calculate_directory_sizes("test_input.txt", &mut directories);
    assert_eq!(sum_large_directories(&directories, 100_000), 95437);
    assert_eq!(find_smallest_sufficient_directory(&mut directories, 30000000), 24933642);

    let mut directories: Vec<u128> = vec!();
    calculate_directory_sizes("input.txt", &mut directories);
    println!("{}", sum_large_directories(&directories, 100_000));
    println!("{}", find_smallest_sufficient_directory(&mut directories, 30000000));
}

fn calculate_directory_sizes(file: &str, directories: &mut Vec<u128>) {
    let input_contents = fs::read_to_string(file).expect("Error reading file");
    let mut lines = input_contents.lines();

    if lines.next().unwrap() != "$ cd /" {
        panic!();
    }

    let mut directory: Vec<&str> = vec!["root"];
    let mut directory_size: u128 = 0;
    let mut hash_map: HashMap<String, u128> = HashMap::new();
    loop {
        let cmd_option = lines.next();
        if cmd_option == None {
            break;
        }
        let cmd = cmd_option.unwrap();

        if cmd == "$ cd .." {
            directory.pop();
            hash_map
                .entry(directory.clone().join("/"))
                .and_modify(|size| {
                    *size += directory_size;
                })
                .or_insert(directory_size);
            directory_size = *hash_map.get(&directory.join("/")).unwrap();
        } else if cmd == "$ ls" {
            while lines.clone().peekable().peek().unwrap_or(&" ").split_at(1).0 != "$" {
                let item = lines.next().unwrap_or(&" ");
                if item == " " {
                    break;
                }
                if item.split_at(3).0 != "dir" {
                    directory_size += u128
                        ::from_str_radix(item.clone().split(" ").next().unwrap(), 10)
                        .unwrap();
                }
            }
            hash_map
                .entry(directory.join("/"))
                .and_modify(|size| {
                    *size += directory_size;
                })
                .or_insert(directory_size);
        } else {
            hash_map.entry(directory.join("/"));
            directory.push(cmd.split_at(5).1);
            directory_size = 0;
        }
    }

    while directory.len() > 1 {
        directory.pop();
        hash_map
            .entry(directory.clone().join("/"))
            .and_modify(|size| {
                *size += directory_size;
            })
            .or_insert(directory_size);
        directory_size = *hash_map.get(&directory.clone().join("/")).unwrap();
    }

    for (_, value) in hash_map.iter() {
        directories.push(*value);
    }
}

fn sum_large_directories(directories: &Vec<u128>, max_size: u128) -> u128 {
    let mut sum: u128 = 0;
    for size in directories {
        if *size <= max_size {
            sum += size;
        }
    }
    return sum;
}

fn find_smallest_sufficient_directory(directories: &mut Vec<u128>, required_space: u128) -> u128 {
    directories.sort();
    let iter_dir = directories.iter().rev();

    let necessary_deletion = required_space - (70000000u128 - iter_dir.clone().next().unwrap());

    return *iter_dir
        .filter(|x| x >= &&necessary_deletion)
        .last()
        .unwrap();
}