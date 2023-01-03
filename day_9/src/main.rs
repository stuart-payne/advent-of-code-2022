use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

const INPUT_FILE: &str = "./input.txt";

fn main() {
    println!("{}", part_one(INPUT_FILE));
    println!("{}", part_two(INPUT_FILE));
}

#[derive(Clone)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn change(&mut self, dir: &Direction) {
        match dir {
            Direction::Right => {
                self.x += 1;
            }
            Direction::Left => {
                self.x -= 1;
            }
            Direction::Down => {
                self.y -= 1;
            }
            Direction::Up => {
                self.y += 1;
            }
        }
    }

    fn chase(&mut self, head: &Position) -> Option<String> {
        let x_diff: i32 = head.x - self.x;
        let y_diff: i32 = head.y - self.y;

        if x_diff.abs() < 2 && y_diff.abs() < 2 {
            return None;
        }

        if x_diff == 0 {
            if y_diff.is_negative() {
                self.y -= 1;
            } else {
                self.y += 1;
            }
            return Some(self.to_string());
        }

        if y_diff == 0 {
            if x_diff.is_negative() {
                self.x -= 1;
            } else {
                self.x += 1;
            }
            return Some(self.to_string());
        }

        if x_diff.is_positive() {
            self.x += 1;
        } else {
            self.x -= 1;
        }

        if y_diff.is_positive() {
            self.y += 1;
        } else {
            self.y -= 1;
        }
        Some(self.to_string())
    }

    fn to_string(&self) -> String {
        self.x.to_string() + "," + &self.y.to_string()
    }

    fn new() -> Self {
        Position { x: 0, y: 0 }
    }
}

enum Direction {
    Right,
    Left,
    Up,
    Down,
}

fn parse_line(line: &str) -> (Direction, u8) {
    let command: Vec<&str> = line.split(" ").collect();
    let dir: Direction;
    dir = match command[0] {
        "R" => Direction::Right,
        "L" => Direction::Left,
        "D" => Direction::Down,
        "U" => Direction::Up,
        _ => panic!("AAAAAAAAAAAHHHHHH"),
    };
    (dir, command[1].parse::<u8>().expect("not a digit"))
}

fn part_one(input_file: &str) -> usize {
    let file = File::open(input_file).expect("not a file");
    let lines = BufReader::new(file).lines();

    let mut head = Position { x: 0, y: 0 };

    let mut tail = Position { x: 0, y: 0 };
    let mut positions: HashSet<String> = HashSet::new();
    positions.insert("0,0".to_string());

    for line in lines {
        let (dir, moves) = parse_line(&line.expect("line failed"));
        for _ in 0..moves {
            head.change(&dir);
            let tail_val = tail.chase(&head);
            match tail_val {
                Some(tail_val) => {
                    positions.insert(tail_val);
                }
                None => {}
            }
        }
    }
    positions.len()
}

fn part_two(input_file: &str) -> usize {
    let file = File::open(input_file).expect("not a file");
    let lines = BufReader::new(file).lines();

    let mut head = Position { x: 0, y: 0 };

    let mut tails: Vec<Position> = Vec::new();

    for _ in 0..9 {
        tails.push(Position::new());
    }

    let mut positions: HashSet<String> = HashSet::new();
    positions.insert("0,0".to_string());

    for line in lines {
        let (dir, moves) = parse_line(&line.expect("line failed"));
        for _ in 0..moves {
            head.change(&dir);
            for i in 0..tails.len() {
                if i == 0 {
                    tails[0].chase(&head);
                } else {
                    let leader = tails[i - 1].clone();
                    let tail_val = tails[i].chase(&leader);
                    if i == tails.len() - 1 {
                        match tail_val {
                            Some(tail_val) => {
                                positions.insert(tail_val);
                            }
                            None => {}
                        }
                    }
                }
            }
        }
    }
    positions.len()
}

#[cfg(test)]
mod test {
    use crate::{part_one, part_two};
    #[test]
    fn part_one_test() {
        assert_eq!(part_one("./test.txt"), 13);
    }
    #[test]
    fn part_two_test() {
        assert_eq!(part_two("./test.txt"), 1);
    }
    #[test]
    fn part_two_test_two() {
        assert_eq!(part_two("./test_two.txt"), 36);
    }
}
