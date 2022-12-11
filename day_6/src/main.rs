use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

const INPUT_PATH: &str = "./input.txt";

fn main() {
    let input_file = File::open(INPUT_PATH).expect("PATH WRONG");
    let mut reader = BufReader::new(input_file);
    let mut input = String::new();
    reader
        .read_line(&mut input)
        .expect("YOU DON@T UNDERSTAND SHIT IDIOT");
    println!("part_one: {}", part_one(&input, 4).expect("NO RESULT"));
    println!("part_two: {}", part_one(&input, 14).expect("NO RESULT"));
}

fn part_one(input: &String, message_size: usize) -> Option<usize> {
    let mut start_cursor: usize = 0;
    let mut end_cursor: usize = message_size - 1;
    let mut success = false;
    let mut unique_letters: HashSet<char> = HashSet::new();
    let mut successful_count = 0;
    while end_cursor <= input.len() {
        let word = &input[start_cursor..=end_cursor];
        for c in word.chars() {
            if unique_letters.contains(&c) {
                break;
            }
            unique_letters.insert(c);
            successful_count += 1;
        }
        if successful_count == message_size {
            success = true;
            break;
        }
        unique_letters.clear();
        successful_count = 0;
        start_cursor += 1;
        end_cursor += 1;
    }
    if success {
        Some(end_cursor + 1)
    } else {
        None
    }
}

#[cfg(test)]
mod test {
    use crate::part_one;
    const TEST_CASES: [(&str, usize); 5] = [
        ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7),
        ("nppdvjthqldpwncqszvftbrmjlhg", 6),
        ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5),
        ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10),
        ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11),
    ];
    const PART_TWO_TEST_CASES: [(&str, usize); 5] = [
        ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 19),
        ("nppdvjthqldpwncqszvftbrmjlhg", 23),
        ("bvwbjplbgvbhsrlpgdmjqwftvncz", 23),
        ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 29),
        ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 26),
    ];
    #[test]
    fn part_one_test() {
        for (input, expected) in TEST_CASES.iter() {
            assert_eq!(
                part_one(&input.to_string(), 4).expect("NOTHING RETURNED"),
                *expected
            );
        }
    }

    #[test]
    fn part_two_test() {
        for (input, expected) in PART_TWO_TEST_CASES.iter() {
            assert_eq!(
                part_one(&input.to_string(), 14).expect("NOTHING RETURNED"),
                *expected
            );
        }
    }
}
