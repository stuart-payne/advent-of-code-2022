use std::{
    fs::File,
    io::{BufRead, BufReader },
};

const FILE_INPUT: &str = "input.txt";
const CYCLE_START: i32  = 20;
const CYCLE_STEP: i32 = 40;
const LIT: char = '#';
const UNLIT: char = '.';

fn main() {
    println!("{}", calc_signal(FILE_INPUT, CYCLE_STEP, CYCLE_START));
}

fn calc_signal(input: &str, cycle_step: i32, cycle_start: i32) -> i32 {
    let mut current_cycle: i32 = 0;
    let mut current_strength: i32 = 1;
    let mut acc_strength: i32 = 0;
    let mut sprite_starting_position: i32 = 1;
    BufReader::new(File::open(input).expect("OI BRUV YOU MAD OR WUT"))
        .lines()
        .for_each(|l| {
            let (cycles, strength) = parse_instruction(l.unwrap());
            for _ in 0..cycles {
                current_cycle += 1;
                let mut pixel_on_line = current_cycle % cycle_step;
                if pixel_on_line == 0 {
                    pixel_on_line = cycle_step;
                }
                if (sprite_starting_position..=sprite_starting_position + 2).contains(&pixel_on_line) {
                    print!("{}", LIT);
                } else {
                    print!("{}", UNLIT);
                }
                if pixel_on_line == cycle_step {
                    print!("\n");
                }
                if current_cycle % cycle_step == cycle_start {
                    acc_strength += current_cycle * current_strength;
                }
            }
            current_strength += strength;
            sprite_starting_position += strength;
            
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
        assert_eq!(calc_signal("./test_input.txt", 40, 20), 13140);
    }
}

