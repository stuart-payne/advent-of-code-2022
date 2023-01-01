use std::{
    collections::{HashMap, VecDeque},
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug)]
struct Directory {
    children: Vec<String>,
    size: u64,
}

const INPUT_PATH: &str = "input.txt";

fn main() {
    let (one, two) = part_one(INPUT_PATH);
    println!("PART ONE: {}", one);
    println!("PART TWO: {}", two);
}

fn part_one(path: &str) -> (u64, u64) {
    let file = File::open(path).expect("file open failed");
    let lines = BufReader::new(file).lines().map(|l| l.expect("(:"));
    let mut directories: HashMap<String, Directory> = HashMap::from([(
        "/".to_string(),
        Directory {
            size: 0,
            children: Vec::new(),
        },
    )]);
    let mut cwd: VecDeque<String> = VecDeque::new();

    for line in lines {
        if line.starts_with("$ cd") {
            process_cd(&line, &mut cwd);
        } else if line.starts_with("dir") {
            process_dir(&line, &cwd, &mut directories)
        } else if line.starts_with(|x: char| x.is_numeric()) {
            process_file(&line, &cwd, &mut directories)
        }
    }

    (
        count_result(&directories),
        count_result_part_two(&directories),
    )
}

fn count_result(directories: &HashMap<String, Directory>) -> u64 {
    let mut count: u64 = 0;
    for (name, dir) in directories.iter() {
        let total_count = get_count(&dir, &directories);
        if total_count <= 100000 {
            count += total_count;
        }
    }
    count
}

const MAX_SPACE: u64 = 70000000;

fn count_result_part_two(directories: &HashMap<String, Directory>) -> u64 {
    let main = directories.get("/").expect("FUCK ME SIDEWAYS");
    println!("TOTAL: {:?}", main);
    let remaining_space = MAX_SPACE - get_count(main, &directories);
    println!("TOTAL: {}", remaining_space);
    let mut smallest: u64 = u64::MAX;
    for (name, dir) in directories.iter() {
        let total_count = get_count(&dir, &directories);
        if remaining_space + total_count >= 30_000_000 && total_count < smallest {
            smallest = total_count;
        }
    }
    smallest
}

fn get_cwd_name(cwd: &VecDeque<String>) -> String {
    cwd.iter()
        .fold("/".to_string(), |acc, val| acc + val + &"/".to_string())
}

fn process_cd(cmd: &str, cwd: &mut VecDeque<String>) {
    let segments = cmd.split(' ').collect::<Vec<&str>>();
    let cmd = segments.iter().nth(segments.len() - 1).expect("nth failed");
    match *cmd {
        ".." => {
            cwd.pop_back();
        }
        "/" => {
            cwd.clear();
        }
        _ => {
            cwd.push_back(cmd.to_string());
        }
    };
}

fn process_dir(cmd: &str, cwd: &VecDeque<String>, directories: &mut HashMap<String, Directory>) {
    let segments: Vec<&str> = cmd.split(' ').collect();
    let dir_name = segments
        .iter()
        .nth(segments.len() - 1)
        .expect("dir name fail");
    let current_dir = get_cwd_name(cwd);
    let full_name = current_dir.clone() + &dir_name.to_string() + "/";
    if !directories.contains_key(&full_name) {
        directories.insert(
            full_name.clone(),
            Directory {
                children: Vec::new(),
                size: 0,
            },
        );
        directories
            .get_mut(&current_dir)
            .expect("current dir fail")
            .children
            .push(full_name.to_string().clone());
    }
}

fn process_file(cmd: &str, cwd: &VecDeque<String>, directories: &mut HashMap<String, Directory>) {
    let segments: Vec<&str> = cmd.split(' ').collect();
    let size = segments[0].parse::<u64>().expect("not a number");
    let name = get_cwd_name(cwd);
    directories
        .get_mut(&name)
        .expect("dic look up wrong")
        .size += size;
}

fn get_count(dir: &Directory, values: &HashMap<String, Directory>) -> u64 {
    let mut total = dir.size;
    for child in &dir.children {
        let child_dir = values.get(child).expect("child look up failed");
        total += get_count(&child_dir, &values);
    }
    total
}

#[cfg(test)]
mod test {
    use crate::part_one;
    const TEST_INPUT: &str = "test.txt";

    #[test]
    fn part_one_test() {
        let (one, two) = part_one(TEST_INPUT); 
        assert_eq!(one, 95437);
        assert_eq!(two, 24933642);
    }

    mod process_cd {
        use crate::process_cd;
        use std::collections::VecDeque;
        #[test]
        fn one() {
            let mut cwd: VecDeque<String> = VecDeque::new();
            /* d
             * a
             * c
             * ..
             * /
             * d
             * expected d
             */
            process_cd("$ cd d", &mut cwd);
            process_cd("$ cd a", &mut cwd);
            process_cd("$ cd c", &mut cwd);
            process_cd("$ cd ..", &mut cwd);
            process_cd("$ cd /", &mut cwd);
            process_cd("$ cd d", &mut cwd);
            assert_eq!(*cwd.back().unwrap(), 'd'.to_string());
        }

        #[test]
        fn two() {
            let mut cwd: VecDeque<String> = VecDeque::new();
            /* d
             * a
             * c
             * ..
             * f
             * ..
             * expected a
             */
            process_cd("$ cd d", &mut cwd);
            process_cd("$ cd a", &mut cwd);
            process_cd("$ cd c", &mut cwd);
            process_cd("$ cd ..", &mut cwd);
            process_cd("$ cd f", &mut cwd);
            process_cd("$ cd ..", &mut cwd);
            assert_eq!(*cwd.back().unwrap(), 'a'.to_string());
        }
    }

    mod process_dir {
        use crate::{process_dir, Directory};
        use std::collections::{HashMap, VecDeque};

        #[test]
        fn one() {
            let cwd: VecDeque<String> = VecDeque::from(["d".to_string()]);
            let mut directories: HashMap<String, Directory> = HashMap::from([(
                "d".to_string(),
                Directory {
                    size: 0,
                    children: Vec::new(),
                },
            )]);
            process_dir("dir e", &cwd, &mut directories);
            process_dir("dir f", &cwd, &mut directories);
            assert_eq!(directories.get(&"e".to_string()).is_some(), true);
            assert_eq!(directories.get(&"f".to_string()).is_some(), true);
            let parent = directories.get(&"d".to_string()).unwrap();
            assert_eq!(parent.children.contains(&"e".to_string()), true);
            assert_eq!(parent.children.contains(&"f".to_string()), true);
        }
    }

    mod process_file {
        use crate::{process_file, Directory};
        use std::collections::{HashMap, VecDeque};

        #[test]
        fn one() {
            let cwd: VecDeque<String> = VecDeque::from(["d".to_string()]);
            let mut directories: HashMap<String, Directory> = HashMap::from([(
                "d".to_string(),
                Directory {
                    size: 0,
                    children: Vec::new(),
                },
            )]);
            process_file("100 j", &cwd, &mut directories);
            process_file("100 j", &cwd, &mut directories);
            process_file("100 j", &cwd, &mut directories);
            process_file("100 j", &cwd, &mut directories);
            process_file("100 j", &cwd, &mut directories);
            assert_eq!(directories.get(&"d".to_string()).unwrap().size, 500);
        }
    }

    mod get_count {
        use crate::{count_result, get_count, Directory};
        use std::collections::HashMap;

        #[test]
        fn one() {
            let mut directories: HashMap<String, Directory> = HashMap::from([
                (
                    "d".to_string(),
                    Directory {
                        size: 1,
                        children: Vec::from(["e".to_string(), "f".to_string()]),
                    },
                ),
                (
                    "e".to_string(),
                    Directory {
                        size: 2,
                        children: Vec::from(["g".to_string()]),
                    },
                ),
                (
                    "f".to_string(),
                    Directory {
                        size: 3,
                        children: Vec::new(),
                    },
                ),
                (
                    "g".to_string(),
                    Directory {
                        size: 4,
                        children: Vec::from(["h".to_string()]),
                    },
                ),
                (
                    "h".to_string(),
                    Directory {
                        size: 5,
                        children: Vec::new(),
                    },
                ),
            ]);
            assert_eq!(
                get_count(directories.get(&"d".to_string()).unwrap(), &directories),
                15
            );
            assert_eq!(count_result(&mut directories), 43);
        }
    }
}
