use std::{
    fs::File,
    io::{BufRead, BufReader },
};

const FILE_INPUT: &str = "input.txt";
const CYCLE_START: i32  = 20;
const CYCLE_STEP: i32 = 40;

fn main() {
    println!("{}", calc_signal(FILE_INPUT));
}

fn calc_signal(input: &str) -> i32 {
    let mut current_cycle: i32 = 0;
    let mut current_strength: i32 = 1;
    let mut acc_strength: i32 = 0;
    BufReader::new(File::open(input).expect("OI BRUV YOU MAD OR WUT"))
        .lines()
        .for_each(|l| {
            let (cycles, strength) = parse_instruction(l.unwrap());
            for _ in 0..cycles {
                current_cycle += 1;
                if current_cycle % CYCLE_STEP == CYCLE_START {
                    println!("{}", current_cycle);
                    acc_strength += current_cycle * current_strength;
                }
            }
            current_strength += strength;
            
        });
    acc_strength
}
 fn parse_instruction(line: String) -> (i32, i32) {
     match line.starts_with("a") {
         true => {
            (2, line.split(" ").nth(1).unwrap().parse::<i32>().expect("instruction parse failed"))
         },
         false => {
             (1, 0)
         }
     }
 }

#[cfg(test)]
mod test {
    use crate::calc_signal;

    #[test]
    fn part_one() {
        assert_eq!(calc_signal("./test_input.txt"), 13140);
    }
}

