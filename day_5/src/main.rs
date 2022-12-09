use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

const FILE_INPUT: &str = "./input.txt";
fn main() {
    println!("PART ONE: {}", part_one(FILE_INPUT));
    println!("PART TWO: {}", part_two(FILE_INPUT));
}

fn part_one(file_input: &str) -> String {
    let file = File::open(file_input).expect("WRONG PATH AGAIN SERIOUSLY WTF?!?!?");
    let first_pass = BufReader::new(&file)
        .lines()
        .map(|line| line.expect("error reading line"));
    let mut stacks: HashMap<char, VecDeque<char>> = HashMap::new();
    // build stacks
    let mut positions: Vec<(char, usize)> = Vec::new();

    for line in first_pass.filter(|x| x.starts_with(" 1")) {
        let mut position = 0;
        for c in line.chars() {
            if c.is_numeric() {
                positions.push((c, position));
            }
            position += 1;
        }
        break;
    }

    for (position, _) in positions.iter() {
        stacks.insert(*position, VecDeque::new());
    }
    let new_file = File::open(file_input).expect("WRONG PATH AGAIN SERIOUSLY WTF?!?!?");

    let lines = BufReader::new(new_file)
        .lines()
        .map(|line| line.expect("error reading line"));

    for line in lines {
        if line.starts_with(" 1") {
            break;
        }
        let chars: Vec<char> = line.chars().collect();
        for (character, position) in positions.iter() {
            let val = *chars.get(*position).expect("FAILED INITING BOXES");
            if val.is_alphabetic() {
                stacks
                    .get_mut(character)
                    .expect("FAILED STACK LOOKUP")
                    .push_back(val);
            }
        }
    }

    let anova_file = File::open(file_input).expect("WRONG PATH AGAIN SERIOUSLY WTF?!?!?");

    let instructions = BufReader::new(anova_file)
        .lines()
        .map(|line| line.expect("(:"));
    for instruction in instructions.filter(|x| x.starts_with('m')) {
        let mut hmm = instruction.split(' ');
        let amount = hmm
            .nth(1)
            .expect("?!?!?!?!!?")
            .parse::<u32>()
            .expect("not a digit bruv");
        let from = hmm.nth(1).expect("oh ffs");
        let to = hmm.nth(1).expect("dear lord this is cursed");

        for _ in 0..amount {
            let box_thing = stacks
                .get_mut(&from.chars().next().expect("dumb hack"))
                .expect("stacks lookup failed")
                .pop_front()
                .expect("nothing ere mate");
            stacks
                .get_mut(&to.chars().next().expect("dumb hack"))
                .expect("anova stacks fail")
                .push_front(box_thing);
        }
    }

    let result = positions
        .iter()
        .map(|(position, _)| stacks.get(&position).expect("lol").front());
    result.filter(|x| x.is_some()).map(|x| x.unwrap()).collect()
}

/*
 * it's 1am in the morning so i copy and pasted because i should really be asleep
 */

fn part_two(file_input: &str) -> String {
    let file = File::open(file_input).expect("WRONG PATH AGAIN SERIOUSLY WTF?!?!?");
    let first_pass = BufReader::new(&file)
        .lines()
        .map(|line| line.expect("error reading line"));
    let mut stacks: HashMap<char, VecDeque<char>> = HashMap::new();
    // build stacks
    let mut positions: Vec<(char, usize)> = Vec::new();

    for line in first_pass.filter(|x| x.starts_with(" 1")) {
        let mut position = 0;
        for c in line.chars() {
            if c.is_numeric() {
                positions.push((c, position));
            }
            position += 1;
        }
        break;
    }

    for (position, _) in positions.iter() {
        stacks.insert(*position, VecDeque::new());
    }
    let new_file = File::open(file_input).expect("WRONG PATH AGAIN SERIOUSLY WTF?!?!?");

    let lines = BufReader::new(new_file)
        .lines()
        .map(|line| line.expect("error reading line"));

    for line in lines {
        if line.starts_with(" 1") {
            break;
        }
        let chars: Vec<char> = line.chars().collect();
        for (character, position) in positions.iter() {
            let val = *chars.get(*position).expect("FAILED INITING BOXES");
            if val.is_alphabetic() {
                stacks
                    .get_mut(character)
                    .expect("FAILED STACK LOOKUP")
                    .push_back(val);
            }
        }
    }

    let anova_file = File::open(file_input).expect("WRONG PATH AGAIN SERIOUSLY WTF?!?!?");

    let instructions = BufReader::new(anova_file)
        .lines()
        .map(|line| line.expect("(:"));

    let mut mover: VecDeque<char> = VecDeque::new();
    for instruction in instructions.filter(|x| x.starts_with('m')) {
        let mut hmm = instruction.split(' ');
        let amount = hmm
            .nth(1)
            .expect("?!?!?!?!!?")
            .parse::<u32>()
            .expect("not a digit bruv");
        let from = hmm.nth(1).expect("oh ffs");
        let to = hmm.nth(1).expect("dear lord this is cursed");

        for _ in 0..amount {
            let box_thing = stacks
                .get_mut(&from.chars().next().expect("dumb hack"))
                .expect("stacks lookup failed")
                .pop_front()
                .expect("nothing ere mate");
            mover.push_front(box_thing);
        }
        for box_thing in mover.iter() {
            stacks
                .get_mut(&to.chars().next().expect("dumb hack"))
                .expect("stacks lookup failed")
                .push_front(*box_thing);
        }
        mover.clear();
    }

    let result = positions
        .iter()
        .map(|(position, _)| stacks.get(&position).expect("lol").front());
    result.filter(|x| x.is_some()).map(|x| x.unwrap()).collect()
}

#[cfg(test)]
mod test {
    use crate::{part_one, part_two};
    const TEST_INPUT: &str = "./test.txt";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(TEST_INPUT), "CMZ");
    }
    #[test]
    fn test_part_two() {
        assert_eq!(part_two(TEST_INPUT), "MCD");
    }
}
