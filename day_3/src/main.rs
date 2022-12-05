use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};

const FILE_PATH: &str = "./twat.txt";
// ASCII starting value with the add 26 from the task
const UPPERCASE_ASCII_VALUE: u8 = 64 - 26;
const LOWERSCASE_ASCII_VALUE: u8 = 96;
/*
    for c in "ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string().chars() {
        println!("{} : {}", c, get_score(&c));
    }
    for c in "abcdefghijklmnopqrstuvwxyz".to_string().chars() {
        println!("{} : {}", c, get_score(&c));
    }
*/

fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let file = File::open(FILE_PATH).expect("FILE PATH ERROR");
    let lines = io::BufReader::new(file).lines();

    let mut first_comp: HashSet<char> = HashSet::new();
    let mut second_comp: HashSet<char> = HashSet::new();
    let mut count: u32 = 0;
    for maybe_line in lines {
        if let Ok(line) = maybe_line {
            let first_compartment = &line[0..line.len() / 2];
            let second_compartment = &line[line.len() / 2..line.len()];
            for f in first_compartment.chars() {
                first_comp.insert(f);
            }

            for s in second_compartment.chars() {
                second_comp.insert(s);
            }

            for i in first_comp.intersection(&second_comp) {
                count += get_score(&i);
            }

            first_comp.clear();
            second_comp.clear();
        }
    }
    println!("{}", count);
}

fn part_two() {
    let file = File::open(FILE_PATH).expect("FILE PATH ERROR");
    let mut lines = io::BufReader::new(file).lines();

    let mut count: u32 = 0;

    while let Some(first_elf_line) = lines.next() {
        let first_elf: HashSet<char> = first_elf_line.unwrap().chars().into_iter().clone().collect();
        let second_elf: HashSet<char> = lines.next().unwrap().unwrap().chars().into_iter().clone().collect();
        let third_elf: HashSet<char> = lines.next().unwrap().unwrap().chars().into_iter().clone().collect();

        for inter in first_elf.intersection(&second_elf) {
            if third_elf.contains(inter) {
                count += get_score(inter);
                break;
            }
        }
    }

    println!("{}", count);
}

fn get_score(item: &char) -> u32 {
    let mut buffer: [u8; 4] = [0; 4];
    item.encode_utf8(&mut buffer);
    let score_val: u8;
    if item.is_ascii_uppercase() {
        score_val = UPPERCASE_ASCII_VALUE;
    } else if item.is_ascii_lowercase() {
        score_val = LOWERSCASE_ASCII_VALUE;
    } else {
        panic!("NOT A LETTER FUCKWIT");
    }
    let score = buffer[0] - score_val;
    u32::from(score)
}


#[cfg(test)]
mod tests {
    use crate::get_score;

    #[test]
    fn get_score_one() {
        assert_eq!(16, get_score(&'p'));
    }

    #[test]
    fn get_score_two() {
        assert_eq!(38, get_score(&'L'));
    }
    #[test]
    fn get_score_three() {
        assert_eq!(42, get_score(&'P'));
    }

    #[test]
    fn get_score_four() {
        assert_eq!(22, get_score(&'v'));
    }

    #[test]
    fn get_score_five() {
        assert_eq!(20, get_score(&'t'));
    }

    #[test]
    fn get_score_six() {
        assert_eq!(19, get_score(&'s'));
    }
}
