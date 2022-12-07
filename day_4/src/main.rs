use std::fs::File;
use std::io::{self, BufRead};

const FILE_PATH: &str = "./input.txt";

#[derive(Debug)]
struct Elf {
    lower: u32,
    higher: u32,
}

fn main() {
    let (part_one, part_two) = i_hate_elves(FILE_PATH);
    println!("PART ONE: {}", part_one);
    println!("PART TWO: {}", part_two);
}

fn i_hate_elves(file_path: &str) -> (u32, u32) {
    let file = File::open(file_path).expect("FILE PATH ERROR");
    let lines = io::BufReader::new(file).lines();

    let mut part_one_count = 0;
    let mut part_two_count = 0;
    for possible_line in lines {
        if let Ok(line) = possible_line {
            let ranges: Vec<&str> = line.split(",").collect();
            let mut first_elf_iter = ranges[0]
                .split('-')
                .map(|x| x.parse::<u32>().expect("NOT AN INT"));
            let first_elf = Elf {
                lower: first_elf_iter.next().expect("ITERrEMPTY"),
                higher: first_elf_iter.next().expect("ITERrEMPTY"),
            };
            let mut second_elf_iter = ranges[1]
                .split('-')
                .map(|x| x.parse::<u32>().expect("NOT AN INT"));

            let second_elf = Elf {
                lower: second_elf_iter.next().expect("ITERrEMPTY"),
                higher: second_elf_iter.next().expect("ITERrEMPTY"),
            };

            let mut first_part_complete = false;

            if first_elf.lower <= second_elf.lower && first_elf.higher >= second_elf.higher {
                part_one_count += 1;
                first_part_complete = true;
            }
            if !first_part_complete
                && second_elf.lower <= first_elf.lower
                && second_elf.higher >= first_elf.higher
            {
                part_one_count += 1;
            }

            if first_elf.lower <= second_elf.lower && first_elf.higher >= second_elf.lower {
                part_two_count += 1;
                continue;
            }
            if first_elf.lower <= second_elf.higher && first_elf.higher >= second_elf.higher {
                part_two_count += 1;
                continue;
            }
            if second_elf.lower <= first_elf.lower && second_elf.higher >= first_elf.lower {
                part_two_count += 1;
                continue;
            }
            if second_elf.lower <= first_elf.higher && second_elf.higher >= first_elf.higher {
                part_two_count += 1;
                continue;
            }
        }
    }
    (part_one_count, part_two_count)
}

#[cfg(test)]
mod tests {
    use crate::i_hate_elves;
    const TEST_PATH: &str = "./test.txt";
    const TEST_TWO_PATH: &str = "./test_two.txt";

    #[test]
    fn test_file() {
        assert_eq!(i_hate_elves(TEST_PATH).0, 2);
    }

    #[test]
    fn test_two_file() {
        assert_eq!(i_hate_elves(TEST_TWO_PATH).1, 4);
    }
}
